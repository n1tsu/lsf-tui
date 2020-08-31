use tui::widgets::ListState;


// This structure holds the menu navigation information
pub struct Selection {
    pub categorie_state : ListState,
    pub word_state : ListState,
    cat_num : usize,
    word_num : usize,
    focus_num : usize,
    words_len : usize,
    cat_len : usize,
}

impl Selection {
    pub fn new(size : usize) -> Self {
        let mut res = Self {
            categorie_state : ListState::default(),
            word_state : ListState::default(),
            cat_num : 0,
            word_num : 0,
            focus_num : 0,
            words_len : 0,
            cat_len : size,
        };
        res.categorie_state.select(Some(0));
        res
    }

    // Change focus on left chunk
    pub fn focus_left(&mut self) {
        self.focus_num = 0;
        self.word_num = 0;
        self.word_state.select(None);
    }

    // Change focus on right chunk
    pub fn focus_right(&mut self, w_size : usize) {
        self.words_len = w_size;
        self.focus_num = 1;
        self.word_state.select(Some(self.word_num));
    }

    // Move selection on item above
    pub fn up(&mut self) {
        if self.focus_num == 0 {
            self.cat_num = (self.cat_len + self.cat_num - 1) % self.cat_len;
            self.categorie_state.select(Some(self.cat_num));
        }
        else {
            self.word_num = (self.words_len + self.word_num - 1) % self.words_len;
            self.word_state.select(Some(self.word_num));
        }
    }

    // Move selection on item below
    pub fn down(&mut self) {
        if self.focus_num == 0 {
            self.cat_num = (self.cat_num + 1) % self.cat_len;
            self.categorie_state.select(Some(self.cat_num));
        }
        else {
            self.word_num = (self.word_num + 1) % self.words_len;
            self.word_state.select(Some(self.word_num));
        }
    }

    // Getter for actual categorie index
    pub fn get_categorie_index(&self) -> usize {
        self.cat_num
    }

    // Getter for actual word index
    pub fn get_word_index(&self) -> usize {
       self.word_num
    }
}
