#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FileFormat {
    Elf64,
    Macho64,
}

impl FileFormat {
    pub fn label(&self, original: String) -> String {
        match self {
            Self::Elf64 => original,
            Self::Macho64 => format!("_{}", original),
        }
    }
    pub fn from_str(s: String) -> Option<Self> {
        match s.to_lowercase().as_str() {
            "macho64" => Some(Self::Macho64),
            "elf64" => Some(Self::Elf64),
            _ => None,
        }
    }
}

