#![allow(dead_code)]

#[derive(Debug, Clone)]
pub struct StackAllocator {
    /// Size of each of the variables, ordered by ID
    vars: Vec<u8>,
    /// First `u8` is size, second `usize` is count, ordered by ID
    arrs: Vec<(u8, usize)>,
    alignment: usize,
    initial_offset: usize,
}
impl StackAllocator {
    /// Returns a new, empty `StackAllocator`
    pub fn new(alignment: usize, initial_offset: usize) -> Self {
        Self {
            vars: Vec::new(),
            arrs: Vec::new(),
            alignment,
            initial_offset,
        }
    }
    /// Allocate the locations for the variables and arrays
    /// Must be called in order for `var_location`
    pub fn allocate(self) -> StackAllocation {
        if !self.arrs.is_empty() {
            todo!("Allocate arrays on stacks");
        }
        // Basically the best-fit bin packing algorithm
        // First bin is `alignment - initial_offset`, the rest of the bins are `alignment`
        let mut locations = self.vars.iter().map(|_| 0).collect::<Vec<usize>>();
        let mut stack_depth = self.initial_offset;
        let mut bin_leftover = self.alignment - self.initial_offset;
        let mut ticks = self.vars.iter().map(|_| false).collect::<Vec<bool>>();
        let mut left_over = self.vars.len();
        while left_over != 0 {
            for (var_id, &size) in self.vars.iter().enumerate() {
                let size = size as usize;
                if ticks[var_id] == true {
                    continue;
                }
                if left_over == 0 {
                    break;
                }
                if bin_leftover >= size {
                    bin_leftover -= size;
                    locations[var_id] = stack_depth;
                    stack_depth += size;
                    ticks[var_id] = true;
                    left_over -= 1;
                }
            }
            bin_leftover = self.alignment;
            stack_depth /= self.alignment;
            stack_depth *= self.alignment;
        }
        StackAllocation {
            stack_depth,
            locations,
        }
    }
    /// Add a variable onto the stack, returns the ID of the variable
    pub fn push_var(&mut self, size: u8) -> usize {
        self.vars.push(size);
        self.vars.len() - 1
    }
    /// Add an array onto the stack, returns the ID of the array
    pub fn push_arr(&mut self, size: u8, count: usize) -> usize {
        self.arrs.push((size, count));
        self.vars.len() - 1
    }
}

#[derive(Debug, Clone)]
pub struct StackAllocation {
    stack_depth: usize,
    pub locations: Vec<usize>,
}
impl StackAllocation {
    /// Returns the total depths of the stack
    /// If there is nothing on the stack, will return the initial offset
    pub fn stack_depth(&self) -> usize {
        self.stack_depth
    }
    /// Returns `true` if the stack has no variables or arrays
    /// Must be called after `alloc`
    pub fn is_empty(&self) -> bool {
        self.locations.is_empty()
    }
    /// Return the location of a variable on the stack, relative to the stack base pointer,
    /// aligned according to `stack_depth`
    /// Must be called after `alloc`
    pub fn var_location(&self, id: usize) -> usize {
        self.locations[id]
    }
    /// Return the location of an element of an array on the stack, relative to the stack base pointer,
    /// aligned according to `stack_depth`
    /// Must be called after `alloc`
    pub fn arr_location(&self, id: usize, size: usize, i: usize) -> usize {
        self.locations[id] + i * size
    }
}
