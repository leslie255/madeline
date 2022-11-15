use std::{collections::HashMap, ops::Range};

use crate::{generation::stack_alloc::StackAllocator, ir::Instruction};

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
    /// `u8` is the dtype size
    StackSpace(u8),
    Normal,
    Const([u8; 8]),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct VRegInfo {
    pub external_id: u64,
    /// The type of content inside the VReg, could be either a pointer to a stack space, or a value
    pub content_kind: VRegContentKind,
    /// The first and last appearance of the VReg
    pub lifetime: Range<usize>,
    /// Where the real location of the register is, can be either inside a real register in the CPU
    /// or on the stack
    pub allocation: Option<VRegAlloc>,
}
impl Default for VRegInfo {
    fn default() -> Self {
        Self {
            external_id: 0,
            content_kind: VRegContentKind::Normal,
            lifetime: Default::default(),
            allocation: None,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum VRegAlloc {
    /// Use a real register in place for the virtual register
    RealReg(usize),
    /// VReg is a point to a stack space
    /// `usize` is the Stackspace ID
    StackPtr(usize),
    Const([u8; 8]),
}

impl VRegAlloc {
    pub fn as_real_reg(&self) -> Option<usize> {
        if let Self::RealReg(v) = self {
            Some(*v)
        } else {
            None
        }
    }
    /// Returns the Stackspace ID of the register
    pub fn as_stack_ptr(&self) -> Option<usize> {
        if let Self::StackPtr(v) = self {
            Some(*v)
        } else {
            None
        }
    }
    pub fn as_const(&self) -> Option<[u8; 8]> {
        if let Self::Const(v) = self {
            Some(*v)
        } else {
            None
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
/// State of a virtual register's life at one since step
enum VRegLifeStage {
    Born = 0x10,
    Live,
    Dying,
    Dead = 000,
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
    fn empty(vreg_count: usize) -> Self {
        Self {
            life_stages: (0..vreg_count).map(|_| VRegLifeStage::Dead).collect(),
            has_fn_call: false,
            reg_occupation: Vec::new(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct VRegAllocation<R>
where
    R: Register,
{
    /// Interal ID's for real registers
    /// Index is internal ID, item is external ID
    reg_ids: Vec<R>,
    /// Internal ID's for virtual registers
    vreg_ids: HashMap<u64, usize>,
    /// Information of all of the virtual registers used
    /// Ordered by internal ID's
    vreg_infos: Vec<VRegInfo>,
    /// A big map of the status of the status of every VReg and real registers in every step of one
    /// block
    step_map: Vec<RegStatus>,
}
impl<R> VRegAllocation<R>
where
    R: Register,
{
    /// Create a new empty VRegAllocator with the given size
    pub fn empty(step_count: usize, vreg_count: usize) -> Self {
        let regs = R::caller_saved();
        Self {
            reg_ids: regs,
            vreg_ids: HashMap::with_capacity(vreg_count),
            vreg_infos: Vec::with_capacity(vreg_count),
            step_map: (0..step_count)
                .map(|_| RegStatus::empty(vreg_count))
                .collect(),
        }
    }
    /// Add a new virtual register
    fn add_vreg(&mut self, name: u64, kind: VRegContentKind) {
        let internal_id = self.vreg_infos.len();
        self.vreg_ids.insert(name, internal_id);
        self.vreg_infos.push(VRegInfo {
            external_id: name,
            content_kind: kind,
            lifetime: 0..0,
            allocation: None,
        })
    }
    /// Mark a virtual register alive at `step` as `SingleDay`
    /// `id` is external ID
    /// Will panic if the ID does not exist
    fn mark_alive(&mut self, id: u64, step: usize) {
        let internal_id = *self.vreg_ids.get(&id).unwrap();
        let vreg_info = self.vreg_infos.get_mut(internal_id).unwrap();
        vreg_info.lifetime = step..step;
        self.step_map[step].life_stages[internal_id] = VRegLifeStage::Born;
    }
    /// Extend the lifetime of a virtual register to `step`
    /// Start of the lifetime does not change
    /// If `step` < `start` the function would mark the register alive to the end of the body
    fn mark_alive_until(&mut self, id: u64, end: usize) {
        let internal_id = *self.vreg_ids.get(&id).unwrap();
        let vreg_info = self.vreg_infos.get_mut(internal_id).unwrap();
        let start_index = if vreg_info.lifetime.start == vreg_info.lifetime.end {
            vreg_info.lifetime.end + 1
        } else {
            vreg_info.lifetime.end
        };
        self.step_map[start_index..end]
            .iter_mut()
            .for_each(|status| status.life_stages[internal_id] = VRegLifeStage::Live);
        self.step_map[end].life_stages[internal_id] = VRegLifeStage::Dying;
        vreg_info.lifetime.end = end;
    }
    /// Try to allocate a real register for the VReg, returns the internal ID for the register
    fn try_alloc_real_reg(reg_occupations: &mut Vec<bool>) -> Option<usize> {
        reg_occupations
            .iter_mut()
            .enumerate()
            .find_map(|(i, occupied)| {
                if *occupied {
                    None
                } else {
                    *occupied = true;
                    Some(i)
                }
            })
    }
    /// Generate a register allocator for a block
    pub fn generate_from(body: &Vec<Instruction>, stack_allocator: &mut StackAllocator) -> Self {
        let vreg_count = body.iter().filter(|&i| i.is_def_reg()).count();
        let step_count = body.len();
        let mut allocator = Self::empty(step_count, vreg_count);
        macro_rules! update_reg_lifetime_if_needed {
            ($i: expr, $step: expr) => {
                match $i {
                    Instruction::Reg(_, id) | Instruction::Load { id, dtype: _ } => {
                        allocator.mark_alive_until(*id, $step);
                    }
                    _ => (),
                }
            };
        }
        body.iter()
            .enumerate()
            .for_each(|(step, instr)| match instr {
                Instruction::DefReg { id, rhs } => {
                    allocator.add_vreg(
                        *id,
                        match rhs.as_ref() {
                            Instruction::Alloc(dtype) => {
                                // TODO: dynamic word size
                                VRegContentKind::StackSpace(dtype.size(8))
                            }
                            Instruction::Reg(_, id) => {
                                match allocator.vreg_infos[allocator.vreg_ids[id]].content_kind {
                                    VRegContentKind::StackSpace(_) => VRegContentKind::Normal,
                                    kind => kind,
                                }
                            }
                            Instruction::UInt(_, u) => VRegContentKind::Const(u.to_be_bytes()),
                            Instruction::Int(_, i) => VRegContentKind::Const(i.to_be_bytes()),
                            Instruction::Float(_, f) => VRegContentKind::Const(f.to_be_bytes()),
                            _ => VRegContentKind::Normal,
                        },
                    );
                    allocator.mark_alive(*id, step);
                    update_reg_lifetime_if_needed!(rhs.as_ref(), step);
                }
                Instruction::Store { id, rhs } => {
                    allocator.mark_alive_until(*id, step);
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
                    allocator.step_map[step].has_fn_call = true;
                    for arg in args {
                        update_reg_lifetime_if_needed!(arg, step);
                    }
                }
                Instruction::Label(_) => (),
                instr => panic!("{:?} in root level is invalid", instr),
            });
        allocator.alloc_regs(stack_allocator);
        allocator
    }
    /// Allocate real registers or stack space for the all virtual registers
    fn alloc_regs(&mut self, stack_allocator: &mut StackAllocator) {
        let mut reg_occupation: Vec<bool> = self.reg_ids.iter().map(|_| false).collect();
        for row in &mut self.step_map {
            let life_stages = &row.life_stages;
            for (internal_id, life_stage) in life_stages.iter().enumerate() {
                match life_stage {
                    VRegLifeStage::Born if self.vreg_infos[internal_id].lifetime.len() != 0 => {
                        match self.vreg_infos[internal_id].content_kind {
                            VRegContentKind::StackSpace(dtype_size) => {
                                let stackspace_id = stack_allocator.add_var(dtype_size);
                                self.vreg_infos[internal_id].allocation =
                                    Some(VRegAlloc::StackPtr(stackspace_id));
                            }
                            VRegContentKind::Normal => {
                                if let Some(reg_id) = Self::try_alloc_real_reg(&mut reg_occupation)
                                {
                                    self.vreg_infos[internal_id].allocation =
                                        Some(VRegAlloc::RealReg(reg_id));
                                } else {
                                    todo!("Allocate stack space for virtual register");
                                }
                            }
                            VRegContentKind::Const(val) => {
                                self.vreg_infos[internal_id].allocation =
                                    Some(VRegAlloc::Const(val))
                            }
                        }
                    }
                    VRegLifeStage::Live => (),
                    VRegLifeStage::Dying => {
                        if let Some(VRegAlloc::RealReg(reg)) =
                            self.vreg_infos[internal_id].allocation
                        {
                            reg_occupation[reg] = false;
                        }
                    }
                    VRegLifeStage::Dead => (),
                    _ => (),
                }
            }
            row.reg_occupation = reg_occupation.to_vec();
        }
    }
    #[allow(dead_code)]
    /// Return a register if the VReg is allocated onto a real register
    pub fn get_alloced_reg(&self, id: u64) -> Option<R> {
        let internal_vreg_id = self.vreg_ids[&id];
        let internal_reg_id = self.vreg_infos[internal_vreg_id]
            .allocation?
            .as_real_reg()?;
        Some(self.reg_ids[internal_reg_id])
    }
    /// Returns the Stackspace ID of the VReg
    pub fn get_alloced_stackptr(&self, id: u64) -> Option<usize> {
        let internal_vreg_id = self.vreg_ids[&id];
        let stack_id = self.vreg_infos[internal_vreg_id]
            .allocation?
            .as_stack_ptr()?;
        Some(stack_id)
    }
    pub fn get_alloced_const(&self, id: u64) -> Option<[u8; 8]> {
        let internal_vreg_id = self.vreg_ids[&id];
        self.vreg_infos[internal_vreg_id].allocation?.as_const()
    }

    pub fn for_each_living_reg<F>(&self, step: usize, mut f: F)
    where
        F: FnMut(R),
    {
        self.step_map[step]
            .reg_occupation
            .iter()
            .zip(self.reg_ids.iter())
            .filter(|&(&in_use, _)| in_use)
            .for_each(|(_, &r)| f(r));
    }

    pub fn for_each_living_reg_rev<F>(&self, step: usize, mut f: F)
    where
        F: FnMut(R),
    {
        self.step_map[step]
            .reg_occupation
            .iter()
            .zip(self.reg_ids.iter())
            .filter(|&(&in_use, _)| in_use)
            .rev()
            .for_each(|(_, &r)| f(r));
    }
}

#[allow(dead_code)]
impl<R> VRegAllocation<R>
where
    R: Register + std::fmt::Display,
{
    pub fn print_reg_lifetime_map(&self) {
        if cfg!(debug_assertions) {
            println!("VReg step map:");
            self.vreg_infos
                .iter()
                .for_each(|vreg| print!("\t%{}", vreg.external_id));
            print!("\tcall?");
            self.reg_ids.iter().for_each(|reg| print!("\t{}", reg));
            println!();
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
                if row.has_fn_call {
                    print!("√");
                } else {
                    print!("");
                }
                row.reg_occupation
                    .iter()
                    .for_each(|occupied| print!("\t{}", if *occupied { '√' } else { ' ' }));
                println!();
            });
        }
    }
    pub fn print_reg_infos(&self) {
        if cfg!(debug_assertions) {
            println!("Virtual registers:");
            println!("id:\tkind\tlife\talloc");
            self.vreg_infos.iter().for_each(|info| {
                print!(
                    "{}:\t{}\t{:?}",
                    info.external_id,
                    match info.content_kind {
                        VRegContentKind::StackSpace(_) => "stack",
                        VRegContentKind::Normal => "normal",
                        VRegContentKind::Const(_) => "const",
                    },
                    info.lifetime,
                );
                if let Some(reg_alloc) = info.allocation {
                    match reg_alloc {
                        VRegAlloc::RealReg(reg_id) => println!("\t{}", self.reg_ids[reg_id]),
                        VRegAlloc::StackPtr(loc) => println!("\tstack {}", loc),
                        VRegAlloc::Const(val) => println!("\tconst {}", u64::from_be_bytes(val)),
                    }
                } else {
                    println!("\tNo alloc")
                }
            })
        }
    }
}
