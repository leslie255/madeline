use std::{collections::HashMap, ops::Range};

use crate::ir::Instruction;

pub trait Register
where
    Self: Sized + Copy + Eq + std::fmt::Debug,
{
    fn caller_saved() -> Vec<Self>;
    fn callee_saved() -> Vec<Self>;
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
/// The type of content inside the VReg, could be either a pointer to a stack space, or a value
pub enum VRegContentKind {
    StackSpace,
    Normal,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct VRegInfo<R>
where
    R: Register,
{
    /// The internal ID of a VReg is not the same as the actual name of the register is not the
    /// same as the numeric name used in IR, the interal ID is a consecutive number from 0 in the
    /// order of the their first appearance in the block
    pub internal_id: usize,
    /// The type of content inside the VReg, could be either a pointer to a stack space, or a value
    pub kind: VRegContentKind,
    /// The first and last appearance of the VReg
    pub lifetime: Range<usize>,
    /// Where the real location of the register is, can be either inside a real register in the CPU
    /// or on the stack
    pub allocation: Option<VRegAlloc<R>>,
}
impl<R> Default for VRegInfo<R>
where
    R: Register,
{
    fn default() -> Self {
        Self {
            internal_id: 0,
            kind: VRegContentKind::Normal,
            lifetime: Default::default(),
            allocation: None,
        }
    }
}
impl<R> VRegInfo<R>
where
    R: Register,
{
    fn new(internal_id: usize, kind: VRegContentKind) -> Self {
        Self {
            internal_id,
            kind,
            lifetime: 0..0,
            allocation: None,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum VRegAlloc<R>
where
    R: Register,
{
    RealReg(R),
    // StackSpace,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
/// State of a virtual register's life at one since step
enum VRegLifeStage {
    Born = 0b100,
    Live,
    Dying,
    Dead = 0b000,
}
impl Default for VRegLifeStage {
    fn default() -> Self {
        Self::Dead
    }
}
impl VRegLifeStage {
    #[allow(dead_code)]
    fn is_living(self) -> bool {
        (self as usize) & 0b100 != 0
    }
}

#[derive(Debug, Clone)]
/// Currect status of all the registers in one specific step
/// Aka a "row" in the register lifetime map
struct RegStatus {
    /// Life stages of the VReg's, ordered by their internal ID
    life_stages: Vec<VRegLifeStage>,
    /// Whether or not this step is a function call
    has_fn_call: bool,
    /// Which real register is occupied, ordered by their internal ID
    reg_occupation: Vec<bool>,
    // stack_occupation: ...
}
impl RegStatus {
    fn empty(reg_count: usize, vreg_count: usize) -> Self {
        Self {
            life_stages: (0..vreg_count).map(|_| VRegLifeStage::Dead).collect(),
            has_fn_call: false,
            reg_occupation: (0..reg_count).map(|_| false).collect(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct VRegAllocator<R>
where
    R: Register,
{
    /// Interal ID's for real registers
    reg_ids: Vec<R>,
    /// A big map of the status of the status of every VReg and real registers in every step of one
    /// block
    step_map: Vec<RegStatus>,
    /// Information of all of the virtual registers used
    vreg_infos: HashMap<u64, VRegInfo<R>>,
}
impl<R> VRegAllocator<R>
where
    R: Register,
{
    /// Create a new empty VRegAllocator with the given size
    pub fn empty(step_count: usize, vreg_count: usize) -> Self {
        let regs = R::caller_saved();
        let reg_count = regs.len();
        Self {
            reg_ids: regs,
            step_map: (0..step_count)
                .map(|_| RegStatus::empty(reg_count, vreg_count))
                .collect(),
            vreg_infos: HashMap::<u64, VRegInfo<R>>::new(),
        }
    }
    /// Mark a virtual register alive at `step`
    /// `id` is external ID
    fn mark_alive(&mut self, id: u64, step: usize) {
        let vreg_info = self.vreg_infos.get_mut(&id).unwrap();
        let internal_id = vreg_info.internal_id;
        vreg_info.lifetime = step..step;
        self.step_map[step].life_stages[internal_id] = VRegLifeStage::Born;
    }
    /// Extend the lifetime of a virtual register to `step`
    /// Start of the lifetime does not change
    /// If `step` < `start` the function would mark the register alive to the end of the body
    fn mark_alive_until(&mut self, id: u64, end: usize) {
        let vreg_info = self.vreg_infos.get_mut(&id).unwrap();
        let interal_id = vreg_info.internal_id;
        let start_index = if vreg_info.lifetime.start == vreg_info.lifetime.end {
            vreg_info.lifetime.end + 1
        } else {
            vreg_info.lifetime.end
        };
        self.step_map[start_index..end]
            .iter_mut()
            .for_each(|status| status.life_stages[interal_id] = VRegLifeStage::Live);
        self.step_map[end].life_stages[interal_id] = VRegLifeStage::Dying;
        vreg_info.lifetime.end = end;
    }
    /// Generate a register allocator for a block
    pub fn generate_map_from(&mut self, body: &Vec<Instruction>) {
        macro_rules! update_reg_lifetime_if_needed {
            ($i: expr, $step: expr) => {
                match $i {
                    Instruction::Reg(_, id) | Instruction::Load { id, dtype: _ } => {
                        self.mark_alive_until(*id, $step);
                    }
                    _ => (),
                }
            };
        }
        body.iter()
            .enumerate()
            .for_each(|(step, instr)| match instr {
                Instruction::DefReg { id, rhs } => {
                    let internal_id = self.vreg_infos.len();
                    self.vreg_infos.insert(
                        *id,
                        VRegInfo::new(
                            internal_id,
                            match rhs.as_ref() {
                                Instruction::Alloc(_) => VRegContentKind::StackSpace,
                                _ => VRegContentKind::Normal,
                            },
                        ),
                    );
                    self.mark_alive(*id, step);
                    update_reg_lifetime_if_needed!(rhs.as_ref(), step);
                }
                Instruction::Store { id, rhs } => {
                    self.mark_alive_until(*id, step);
                    update_reg_lifetime_if_needed!(rhs.as_ref(), step);
                }
                Instruction::Ret(Some(ret_val)) => {
                    update_reg_lifetime_if_needed!(ret_val.as_ref(), step);
                }
                Instruction::Add(lhs, rhs)
                | Instruction::Sub(lhs, rhs)
                | Instruction::Mul(lhs, rhs)
                | Instruction::Div(lhs, rhs) => {
                    update_reg_lifetime_if_needed!(lhs.as_ref(), step);
                    update_reg_lifetime_if_needed!(rhs.as_ref(), step);
                }
                Instruction::Ret(None) => (),
                Instruction::Call {
                    ret_type: _,
                    fn_name: _,
                    args,
                } => {
                    self.step_map[step].has_fn_call = true;
                    for arg in args {
                        update_reg_lifetime_if_needed!(arg, step);
                    }
                }
                Instruction::Label(_) => (),
                instr => panic!("{:?} in root level is invalid", instr),
            });
    }
    /// Allocate real registers or stack space for the all virtual registers
    pub fn alloc_real_registers(&mut self) {
        todo!()
    }
}

#[allow(dead_code)]
impl<R> VRegAllocator<R>
where
    R: Register + std::fmt::Display,
{
    pub fn print_reg_lifetime_map(&self) {
        if cfg!(debug_assertions) {
            println!("VReg life stages:");
            let mut interal_external_id_pairs = self
                .vreg_infos
                .iter()
                .map(|(&external_id, info)| (info.internal_id, external_id))
                .collect::<Vec<(usize, u64)>>();
            interal_external_id_pairs.sort_by(|x, y| x.0.cmp(&y.0));
            interal_external_id_pairs
                .iter()
                .for_each(|(external, _)| print!("\t%{}", external));
            self.reg_ids.iter().for_each(|reg| print!("\t{}", reg));
            println!("\tcall?");
            self.step_map.iter().enumerate().for_each(|(row_num, row)| {
                print!("{}\t", row_num);
                row.life_stages
                    .iter()
                    .for_each(|&life_stage| match life_stage {
                        VRegLifeStage::Born => print!("O\t"),
                        VRegLifeStage::Live => print!("|\t"),
                        VRegLifeStage::Dying => print!("X\t"),
                        VRegLifeStage::Dead => print!("\t"),
                    });
                row.reg_occupation
                    .iter()
                    .for_each(|occupied| print!("{}\t", if *occupied { '√' } else { ' ' }));
                println!("{}\t", if row.has_fn_call { '√' } else { ' ' });
            });
        }
    }
    pub fn print_reg_infos(&self) {
        if cfg!(debug_assertions) {
            println!("Virtual registers:");
            println!("id:\tkind\tlife\talloc");
            self.vreg_infos.iter().for_each(|(id, info)| {
                print!(
                    "{}:\t{}\t{:?}",
                    id,
                    match info.kind {
                        VRegContentKind::StackSpace => "stack",
                        VRegContentKind::Normal => "normal",
                    },
                    info.lifetime,
                );
                if let Some(reg_alloc) = info.allocation {
                    match reg_alloc {
                        VRegAlloc::RealReg(reg) => println!("\t{}", reg),
                    }
                } else {
                    println!()
                }
            })
        }
    }
}
