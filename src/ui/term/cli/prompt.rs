// devela::ui::term::prompt
//
//! A versatile CLI prompt.
//

use core::fmt;
use crossterm::{
    cursor::MoveToColumn,
    event::{read, Event, KeyCode, KeyEvent, KeyModifiers},
    terminal::{self, disable_raw_mode, enable_raw_mode, ClearType},
    ExecutableCommand,
};
use std::{
    borrow::Cow,
    io::{stdout, Stdout, Write},
};
use unicode_segmentation::UnicodeSegmentation;
use unicode_width::UnicodeWidthStr;

/* private types */

type PromptResult<T> = core::result::Result<T, Box<dyn std::error::Error>>;

struct PromptOptions {
    /// The suffix after the prompt question.
    prompt_suffix: String,

    /// Maximum allowed number of graphemes.
    max_len: Option<usize>,

    /// Allow an empty input.
    can_be_empty: bool,

    /// Input validator.
    #[allow(clippy::type_complexity)]
    validator:
        Option<Box<dyn Fn(Option<(char, usize)>, &str) -> (bool, Option<Cow<'static, str>>)>>,
}

impl Default for PromptOptions {
    fn default() -> Self {
        Self::new()
    }
}

impl PromptOptions {
    fn new() -> Self {
        Self {
            prompt_suffix: String::from(": "),
            max_len: None,
            can_be_empty: true,
            validator: None,
        }
    }
}
impl fmt::Debug for PromptOptions {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("PromptOptions")
            .field("prompt_suffix", &self.prompt_suffix)
            .field("max_len", &self.max_len)
            .field("can_be_empty", &self.can_be_empty)
            .field(
                "validator",
                if self.validator.is_some() {
                    &"Some(_)"
                } else {
                    &"None"
                },
            )
            .finish()
    }
}

/* public type */

/// A customizable prompt.
#[derive(Debug, Default)]
pub struct Prompt {
    options: PromptOptions,
}

impl Prompt {
    /// Return a new default prompt.
    pub fn new() -> Self {
        Self {
            options: PromptOptions::new(),
        }
    }

    /// Set a prompt suffix. Default: `: `.
    pub fn suffix(mut self, suffix: &str) -> Self {
        suffix.clone_into(&mut self.options.prompt_suffix);
        self
    }

    /// Set the allowed maximum number of extended graphems clusters.
    pub fn max_len(mut self, max: usize) -> Self {
        self.options.max_len = Some(max);
        self
    }

    /// Configure whether the response `can` be empty or not. (true by default).
    pub fn can_be_empty(mut self, can: bool) -> Self {
        self.options.can_be_empty = can;
        self
    }

    /// Configures a validation closure for input characters.
    ///
    /// The closure takes 2 arguments:
    /// 1. A tuple with the character and the position (in graphemes)
    /// 2. The full prompt string.
    ///
    /// The closure must return a tuple of (`bool`, `Option<String>`),
    /// where the bool indicates if the character is valid, and the String
    /// explains why it is not valid.
    pub fn validator<F>(mut self, closure: F) -> Self
    where
        F: Fn(Option<(char, usize)>, &str) -> (bool, Option<Cow<'static, str>>) + 'static,
    {
        self.options.validator = Some(Box::new(closure));
        self
    }

    /* */

    /// Prompts a `message` with `default` input, and returns the response.
    ///
    /// It supports:
    /// - extended grapheme clusters.
    /// - a default string
    /// - delete, backspace.
    /// - cancelling with ctrl + c.
    /// - movement with arrows, ctrl + arrows, end, home.
    pub fn ask(&self, message: &str, default: &str) -> PromptResult<String> {
        let mut stdout = stdout();
        let mut input = default.to_string();
        let mut icursor = UnicodeSegmentation::graphemes(default, true).count();

        let mut msg_printed = false;

        enable_raw_mode()?;
        stdout.execute(MoveToColumn(0))?;
        print!("{}{}{}", message, self.options.prompt_suffix, input);
        stdout.flush()?;
        let msglen = (message.graphemes(true).count()
            + self.options.prompt_suffix.graphemes(true).count()) as u16;
        stdout.execute(MoveToColumn(msglen + icursor as u16))?;

        loop {
            if let Event::Key(KeyEvent {
                code, modifiers, ..
            }) = read()?
            {
                match code {
                    //
                    KeyCode::Enter => {
                        // validate the input.
                        let (is_valid, msg) = {
                            // empty message allowed?
                            if input.is_empty() {
                                if self.options.can_be_empty {
                                    (true, None)
                                } else {
                                    (false, Some(Cow::Borrowed("Can't be empty!")))
                                }
                            // valid response format?
                            } else {
                                let (is_valid, msg) = self
                                    .options
                                    .validator
                                    .as_ref()
                                    .map_or((true, None), |validator| validator(None, &input));
                                (is_valid, msg)
                            }
                        };

                        self.print_validation_msg(
                            &mut stdout,
                            msg.as_deref(),
                            &mut msg_printed,
                            icursor,
                            &input,
                            msglen,
                        )?;

                        if is_valid {
                            disable_raw_mode()?;
                            println!();
                            break;
                        }
                    }

                    KeyCode::Char(c) => {
                        if modifiers.contains(KeyModifiers::CONTROL) && c == 'c' {
                            disable_raw_mode()?;
                            println!();
                            return Err("Input cancelled by user".into());
                        } else {
                            // validate the input
                            let (is_valid_char, is_valid_char_msg) = self
                                .options
                                .validator
                                .as_ref()
                                .map_or((true, None), |validator| {
                                    validator(Some((c, icursor)), &input)
                                });

                            self.print_validation_msg(
                                &mut stdout,
                                is_valid_char_msg.as_deref(),
                                &mut msg_printed,
                                icursor,
                                &input,
                                msglen,
                            )?;

                            // Check we're under the optional max egc limit.
                            let is_in_len = self
                                .options
                                .max_len
                                .map_or(true, |v| input.graphemes(true).count() < v);

                            if is_valid_char && is_in_len {
                                // Find the index where the new character should be inserted
                                let insert_index =
                                    UnicodeSegmentation::grapheme_indices(input.as_str(), true)
                                        .nth(icursor)
                                        .map_or(input.len(), |(i, _)| i);

                                // Convert char to string because it might be a multi-byte grapheme
                                let grapheme = c.to_string();

                                // Insert the grapheme at the correct index
                                input.insert_str(insert_index, &grapheme);

                                // Clear the line and reprint the entire string
                                stdout.execute(MoveToColumn(msglen))?;
                                stdout.execute(terminal::Clear(ClearType::UntilNewLine))?;
                                print!("{}", input);
                                stdout.flush()?;

                                // Increment icursor by 1, which represents one grapheme
                                icursor += 1;

                                // Calculate correct column position for the cursor after insertion
                                let cursor_column = msglen
                                    + input[..insert_index]
                                        .graphemes(true)
                                        .map(|g| UnicodeWidthStr::width(g) as u16)
                                        .sum::<u16>()
                                    + UnicodeWidthStr::width(grapheme.as_str()) as u16;

                                // Move the cursor to the new position
                                stdout.execute(MoveToColumn(cursor_column))?;
                            }
                        }
                    }
                    KeyCode::Backspace => {
                        if icursor > 0 {
                            // Determine the start of the grapheme to be deleted
                            let del_start_index =
                                UnicodeSegmentation::grapheme_indices(input.as_str(), true)
                                    .take(icursor)
                                    .last()
                                    .map_or(0, |(i, _)| i);
                            // Determine the end of the grapheme to be deleted
                            let del_end_index = del_start_index
                                + input[del_start_index..]
                                    .graphemes(true)
                                    .next()
                                    .unwrap()
                                    .len();

                            // Delete the grapheme
                            input.replace_range(del_start_index..del_end_index, "");

                            // Recalculate the cursor position
                            icursor = UnicodeSegmentation::grapheme_indices(input.as_str(), true)
                                .take_while(|&(i, _)| i < del_start_index)
                                .count();

                            // Reprint the input and reset the cursor position
                            stdout.execute(MoveToColumn(msglen))?;
                            stdout.execute(terminal::Clear(ClearType::FromCursorDown))?;
                            print!("{}", input);
                            stdout.execute(MoveToColumn(msglen + icursor as u16))?;
                        }
                    }

                    // Delete the character to the right of the cursor
                    KeyCode::Delete => {
                        if icursor < input.graphemes(true).count() {
                            // Determine the end of the grapheme to be deleted
                            let del_end_index =
                                UnicodeSegmentation::grapheme_indices(input.as_str(), true)
                                    .nth(icursor + 1)
                                    .map_or(input.len(), |(i, _)| i);

                            // Delete the grapheme
                            input.replace_range(icursor..del_end_index, "");

                            // Reprint the input and reset the cursor position
                            stdout.execute(MoveToColumn(msglen))?;
                            stdout.execute(terminal::Clear(ClearType::FromCursorDown))?;
                            print!("{}", input);
                            stdout.execute(MoveToColumn(msglen + icursor as u16))?;
                            stdout.flush()?;
                        }
                    }

                    // Move cursor to the beginning of the input
                    KeyCode::Home => {
                        icursor = 0;
                        let cursor_column = msglen;
                        stdout.execute(MoveToColumn(cursor_column))?;
                    }

                    // Move cursor to the end of the input
                    KeyCode::End => {
                        icursor = input.graphemes(true).count();
                        let cursor_column = msglen
                            + input
                                .graphemes(true)
                                .map(|g| UnicodeWidthStr::width(g) as u16)
                                .sum::<u16>();

                        stdout.execute(MoveToColumn(cursor_column))?;
                    }

                    // Move cursor one word to the left
                    KeyCode::Left if modifiers.contains(KeyModifiers::CONTROL) => {
                        // Move cursor to the beginning of the previous word
                        if icursor > 0 {
                            let grapheme_indices: Vec<_> =
                                UnicodeSegmentation::grapheme_indices(input.as_str(), true)
                                    .collect();
                            let mut target_index = 0;
                            let mut found_non_space = false;

                            for (i, (index, _)) in grapheme_indices.iter().enumerate().rev() {
                                if i < icursor {
                                    if input.as_bytes()[*index] != b' ' {
                                        found_non_space = true;
                                    } else if found_non_space {
                                        target_index = i + 1;
                                        break;
                                    }
                                }
                            }

                            icursor = target_index;
                            let cursor_position = input
                                .graphemes(true)
                                .take(icursor)
                                .map(|g| UnicodeWidthStr::width(g) as u16)
                                .sum::<u16>();
                            stdout.execute(MoveToColumn(msglen + cursor_position))?;
                        }
                    }

                    // Move cursor one word to the right
                    KeyCode::Right if modifiers.contains(KeyModifiers::CONTROL) => {
                        let grapheme_indices: Vec<_> =
                            UnicodeSegmentation::grapheme_indices(input.as_str(), true).collect();
                        let mut target_index = grapheme_indices.len();
                        let mut found_word = false;

                        for (i, (index, _)) in grapheme_indices.iter().enumerate() {
                            if i >= icursor {
                                if input.as_bytes()[*index] != b' ' {
                                    // Found a non-whitespace character.
                                    found_word = true;
                                } else if found_word {
                                    // Found whitespace after a word.
                                    target_index = i;
                                    break;
                                }
                            }
                        }

                        icursor = target_index;
                        let cursor_position = input
                            .graphemes(true)
                            .take(icursor)
                            .map(|g| UnicodeWidthStr::width(g) as u16)
                            .sum::<u16>();
                        stdout.execute(MoveToColumn(msglen + cursor_position))?;
                    }

                    KeyCode::Left => {
                        if icursor > 0 {
                            icursor -= 1;
                            let cursor_position = input
                                .graphemes(true)
                                .take(icursor)
                                .map(|g| UnicodeWidthStr::width(g) as u16)
                                .sum::<u16>();
                            stdout.execute(MoveToColumn(msglen + cursor_position))?;
                        }
                    }
                    KeyCode::Right => {
                        if icursor < UnicodeSegmentation::graphemes(input.as_str(), true).count() {
                            icursor += 1;
                            let cursor_position = input
                                .graphemes(true)
                                .take(icursor)
                                .map(|g| UnicodeWidthStr::width(g) as u16)
                                .sum::<u16>();
                            stdout.execute(MoveToColumn(msglen + cursor_position))?;
                        }
                    }

                    _ => {}
                }
                stdout.flush()?;
            }
        }
        Ok(input)
    }

    // private helper for handling printing the validation message
    fn print_validation_msg(
        &self,
        stdout: &mut Stdout,
        message: Option<&str>,
        msg_printed: &mut bool,
        icursor: usize,
        input: &str,
        msglen: u16,
    ) -> PromptResult<()> {
        // Display validation message or clear the previous one
        if let Some(message) = message {
            stdout.execute(crossterm::cursor::MoveToColumn(0))?;
            print!("\n{}", message);
            stdout.execute(terminal::Clear(ClearType::UntilNewLine))?;
            stdout.execute(crossterm::cursor::MoveUp(1))?;
            *msg_printed = true;
        } else if *msg_printed {
            stdout.execute(crossterm::cursor::MoveDown(1))?;
            stdout.execute(terminal::Clear(ClearType::CurrentLine))?;
            stdout.execute(crossterm::cursor::MoveUp(1))?;
            *msg_printed = false;
        }

        // Calculate the cursor position based on the width of graphemes up to icursor
        let cursor_position = input
            .graphemes(true)
            .take(icursor)
            .map(|g| UnicodeWidthStr::width(g) as u16)
            .sum::<u16>();

        // Add the message length to the cursor position
        let cursor_column = msglen + cursor_position;

        stdout.execute(MoveToColumn(cursor_column))?;

        Ok(())
    }
}
