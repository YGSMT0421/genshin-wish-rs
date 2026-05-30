mod pool_data;
mod pools;
mod status;

#[cfg(test)]
mod tests {
    use super::*;

    mod test_character_pool {
        use super::*;

        trait CharacterPoolEditor {
            fn five_wish_times_editor(&mut self, new: u8);
            fn four_wish_times_editor(&mut self, new: u8);
            fn guaranteed_editor(&mut self, new: bool);
            fn off_banner_times_editor(&mut self, new: u8);
        }

        impl<'a> CharacterPoolEditor for pools::CharacterPool<'a> {
            fn five_wish_times_editor(&mut self, new: u8) {
                self.five_wish_times = new;
            }

            fn four_wish_times_editor(&mut self, new: u8) {
                self.four_wish_times = new;
            }

            fn guaranteed_editor(&mut self, new: bool) {
                self.guaranteed = new;
            }

            fn off_banner_times_editor(&mut self, new: u8) {
                self.off_banner_times = new;
            }
        }
        
        fn setup_pool() -> CharacterPool {
            
        }
    }
}
