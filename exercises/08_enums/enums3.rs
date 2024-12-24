struct Point {
    x: u64,
    y: u64,
}

enum Message {
    Resize { width: u64, height: u64 },
    Move(Point),
    Echo(String),
    ChangeColor(u8, u8, u8),
    Quit,
}

struct State {
    width: u64,
    height: u64,
    position: Point,
    message: String,
    // RGB color composed of red, green and blue.
    color: (u8, u8, u8),
    quit: bool,
}

impl State {
    fn resize(&mut self, width: u64, height: u64) {
        self.width = width;
        self.height = height;
    }

    fn move_position(&mut self, point: Point) {
        self.position = point;
    }

    fn echo(&mut self, s: String) {
        self.message = s;
    }

    fn change_color(&mut self, red: u8, green: u8, blue: u8) {
        self.color = (red, green, blue);
    }

    fn quit(&mut self) {
        self.quit = true;
    }

    fn process(&mut self, message: Message) {
        // TODO: Create a match expression to process the different message
        // variants using the methods defined above.
        let State {
            width,
            height,
            position,
            message: state_message,
            color,
            quit,
        } = self;

        match message {
            Message::Resize {
                width: new_width,
                height: new_height,
            } => {
                *width = new_width;
                *height = new_height;
            }
            Message::ChangeColor(r, g, b) => {
                *color = (r, g, b);
            }
            Message::Move(Point { x, y }) => {
                position.x = x;
                position.y = y;
            }
            Message::Echo(new_message) => *state_message = new_message,
            Message::Quit => *quit = true,
        }
    }
}

fn main() {
    // You can optionally experiment here.
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_match_message_call() {
        let mut state = State {
            width: 0,
            height: 0,
            position: Point { x: 0, y: 0 },
            message: String::from("hello world"),
            color: (0, 0, 0),
            quit: false,
        };

        state.process(Message::Resize {
            width: 10,
            height: 30,
        });
        state.process(Message::Move(Point { x: 10, y: 15 }));
        state.process(Message::Echo(String::from("Hello world!")));
        state.process(Message::ChangeColor(255, 0, 255));
        state.process(Message::Quit);

        assert_eq!(state.width, 10);
        assert_eq!(state.height, 30);
        assert_eq!(state.position.x, 10);
        assert_eq!(state.position.y, 15);
        assert_eq!(state.message, "Hello world!");
        assert_eq!(state.color, (255, 0, 255));
        assert!(state.quit);
    }
}

// | **情況**                           | **是否需要 `*`** | **原因**                                                                                         | **範例**                                                                                                  |
// |-------------------------------------|-------------------|---------------------------------------------------------------------------------------------------|----------------------------------------------------------------------------------------------------------|
// | 擁有型資料（Owned Data）             | 不需要            | 擁有型資料直接操作即可，無需解引用。                                                             | `self.position.x = x;`                                                                                   |
// | 可變引用（`&mut`）                  | 需要              | 必須解引用以訪問和修改引用指向的值。                                                             | `*state_message = new_message;`                                                                          |
// | 不可變引用（`&`），只讀訪問         | 不需要            | Rust 自動解引用不可變引用進行只讀訪問。                                                          | `let length = some_ref.len();`                                                                          |
// | 模式匹配中解構引用                  | 不需要            | Rust 自動解引用模式匹配中的引用，解構後直接訪問值。                                              | `Message::Move(Point { x, y }) => { self.position.x = x; }`                                              |
// | 解構擁有型資料                     | 不需要            | 擁有型資料在模式匹配中解構後直接訪問值，無需解引用。                                             | `let State { width, height, .. } = self;`                                                               |
// | 引用型資料，傳遞給函數時只讀        | 不需要            | 函數會自動解引用不可變引用，無需顯式解引用。                                                    | `println!("{}", &value);`                                                                               |
// | 引用型資料，傳遞給函數並修改        | 需要              | 如果函數參數是需要解引用的資料類型，需顯式解引用。                                              | `let val = *some_mut_ref + 1;`                                                                          |
// | 可變引用，函數參數需要引用          | 不需要            | 當函數參數本身預期接受引用（如 `&mut`），則不需要額外解引用。                                    | `some_function(&mut self.some_field);`                                                                  |
