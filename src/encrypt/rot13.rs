#[derive(Debug)]
pub struct Rot13 {
    pub data: String,
}

impl super::Encryptable for Rot13 {
    // fn encrypt(&self) -> String {
    //     let mut new_string = String::new();
    //     for i in self.0.chars() {
    //         if (i >= 'a' && i < 'n') || (i >= 'A' && i < 'N') {
    //             new_string.push((i as u8 + 13) as char);
    //         } else if (i >= 'n' && i <= 'z') || (i >= 'N' && i <= 'Z') {
    //             new_string.push((i as u8 - 13) as char);
    //         } else {
    //             new_string.push(i);
    //         }
    //     }
    //     new_string
    // }

    fn encrypt(&self) -> String {
        // self.data
        //     .chars()
        //     .map(|ch| {
        //         if (ch >= 'a' && ch < 'n') || (ch >= 'A' && ch < 'N') {
        //             (ch as u8 + 13) as char
        //         } else if (ch >= 'n' && ch < 'z') || (ch >= 'N' && ch < 'Z') {
        //             (ch as u8 - 13) as char
        //         } else {
        //             ch
        //         }
        //     })
        //     .collect()

        self.data
            .chars()
            .map(|ch| match ch {
                'a'..='m' | 'A'..='M' => (ch as u8 + 13) as char,
                'n'..='z' | 'N'..='Z' => (ch as u8 - 13) as char,
                _ => ch,
            })
            .collect()
    }
}
