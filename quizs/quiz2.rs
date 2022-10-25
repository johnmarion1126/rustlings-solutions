// quiz2.rs
// This is a quiz for the following sections:
// - Strings
// - Vecs
// - Move semantics
// - Modules
// - Enums

// Let's build a little machine in form of a function.
// As input, we're going to give a list of strings and commands. These commands
// determine what action is going to be applied to the string. It can either be:
// - Uppercase the string
// - Trim the string
// - Append "bar" to the string a specified amount of times
// The exact form of this will be:
// - The input is going to be a Vector of a 2-length tuple,
//   the first element is the string, the second one is the command.
// - The output element is going to be a Vector of strings.
// Execute `rustlings hint quiz2` or use the `hint` watch subcommand for a hint.

// I AM NOT DONE

pub enum Command {
    Uppercase,
    Trim,
    Append(usize),
}

type TransformerInput = (String, Command);

mod my_module {
    use super::Command;
    use super::TransformerInput;

    fn uppercase(mut val: &String) -> String {
        val.to_uppercase()
    }

    fn trim(mut val: &String) -> String {
        (val.trim()).to_string()
    }

    fn append_bar(mut val: &String, amount: &usize) -> String {
        let bar = "bar".repeat(*amount);
        val.to_owned() + &bar
    }


    pub fn transformer(input: Vec<TransformerInput>) -> Vec<String> {
        
        // TODO: Complete the output declaration!
        let mut output: Vec<String> = vec![];
        
        for (string, command) in input.iter() {
            // TODO: Complete the function body. You can do it!
            match command {
                Command::Uppercase => {
                    output.push(uppercase(string));
                },
                Command::Trim => {
                    output.push(trim(string));
                },
                Command::Append(amount) => {
                    output.push(append_bar(string, amount));
                },
            }
        }
        
        output
    }
}

#[cfg(test)]
mod tests {
    // TODO: What do we have to import to have `transformer` in scope?
    use super::my_module::transformer;
    use super::Command;

    #[test]
    fn it_works() {
        let output = transformer(vec![
            ("hello".into(), Command::Uppercase),
            (" all roads lead to rome! ".into(), Command::Trim),
            ("foo".into(), Command::Append(1)),
            ("bar".into(), Command::Append(5)),
        ]);
        assert_eq!(output[0], "HELLO");
        assert_eq!(output[1], "all roads lead to rome!");
        assert_eq!(output[2], "foobar");
        assert_eq!(output[3], "barbarbarbarbarbar");
    }
}
