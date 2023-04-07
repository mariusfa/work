use crate::actions::Actions;
use std::error::Error;

pub struct ActionsImpl {}

impl Actions for ActionsImpl {
    fn start(&mut self) -> Result<(), Box<dyn Error>> {
        unimplemented!();
    }

    fn status(&mut self) -> Result<(), Box<dyn Error>> {
        unimplemented!()
    }
}

mod config_file {
    use crate::storage::FileOperations;
    use std::error::Error;

    pub fn update(file_operations: &mut dyn FileOperations) -> Result<(), Box<dyn Error>> {
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::{config_file::update};
    use crate::storage::{MockFile, FileOperations};

    #[test]
    fn test_update_single_iteration() {
        let mut mock_file = MockFile::new();
        let result = update(&mut mock_file);
        assert!(result.is_ok());
        assert_eq!(mock_file.read(), "1674967648 1674967648");
    }
}
