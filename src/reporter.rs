use crate::{ast::Range, token::*};
use std::{error::Error, path::PathBuf};

#[derive(Debug)]
struct ParsingError {
    msg: String,
}

impl std::fmt::Display for ParsingError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.msg)
    }
}

impl std::error::Error for ParsingError {}

impl ParsingError {
    pub fn new<S: AsRef<str> + ?Sized>(msg: &S) -> Self {
        Self {
            msg: msg.as_ref().to_string(),
        }
    }
}

pub struct ErrorReporter {
    file: Option<PathBuf>,
    source: String,
    lines: Vec<(usize, usize)>,
}

impl ErrorReporter {
    pub fn new(file: Option<PathBuf>, source: String) -> Self {
        let mut lines: Vec<(usize, usize)> = vec![];
        let mut start = 0_usize;
        for (pos, ch) in source.chars().enumerate() {
            match ch {
                '\n' => {
                    lines.push((start, pos));
                    start = pos + 1;
                }
                _ => (),
            }
        }
        if start < source.len() {
            lines.push((start, source.len()))
        }
        Self {
            file,
            source,
            lines,
        }
    }

    pub fn error_range<S: AsRef<str> + ?Sized>(
        &self,
        range: &dyn Range,
        msg: &S,
    ) -> Result<(), Box<dyn Error>> {
        let r = range.range();
        let len = r.1 - r.0 + 1;

        if let Some((s, _e)) = self.range2lineno(r) {
            self.error(msg, s, r.0, len)
        } else {
            panic!()
        }
    }

    pub fn error_token<S: AsRef<str> + ?Sized>(
        &self,
        message: &S,
        token: &Token,
    ) -> Result<(), Box<dyn Error>> {
        self.error(message, token.line(), token.pos(), token.len())
    }

    // pub fn error_expr<S: AsRef<str> + ?Sized>(&self, message: &S, expr: &Expr) -> Result<(),Box<dyn Error>>  {
    //     self.display_message("error", message);
    //     for &(line, start, len) in &expr.range {
    //         let (lineno, inline_pos) = self.inline_coordinates(line, start);
    //         self.display_fileinfo(lineno, inline_pos);
    //         self.display_line(lineno, inline_pos, len);
    //     }

    // 	Err( ParsingError::new(message).into())
    // }

    pub fn error<S: AsRef<str> + ?Sized>(
        &self,
        message: &S,
        lineno: usize,
        file_pos: usize,
        len: usize,
    ) -> Result<(), Box<dyn Error>> {
        self.report("Error", lineno, message, file_pos, len)
    }

    pub fn report<S: AsRef<str> + ?Sized>(
        &self,
        level: &str,
        lineno: usize,
        message: &S,
        file_pos: usize,
        len: usize,
    ) -> Result<(), Box<dyn Error>> {
        let (lineno, inline_pos) = self.inline_coordinates(lineno, file_pos);

        self.display_message(level, message);

        self.display_fileinfo(lineno, inline_pos);

        self.display_line(lineno, inline_pos, len);
        eprintln!();
        Err(ParsingError::new(message).into())
    }

    // 行内坐标
    #[inline]
    fn inline_coordinates(&self, lineno: usize, file_pos: usize) -> (usize, usize) {
        let mut start = file_pos; // inline index

        for i in 0..(lineno - 1) {
            let len = self.lines[i].1 - self.lines[i].0;
            start = start - (len + 1);
        }

        (lineno, start)
    }

    #[inline]
    fn display_message<S: AsRef<str> + ?Sized>(&self, level: &str, message: &S) {
        eprintln!("{}: {}", level, message.as_ref()); // 报错信息
    }

    #[inline]
    fn display_fileinfo(&self, lineno: usize, inline_pos: usize) {
        if let Some(f) = &self.file {
            // 只要不是控制台的输入,就把文件信息打印出来
            eprintln!("  --> {}:{}:{}", f.display(), lineno, inline_pos + 1);
        }
    }

    fn range2lineno(&self, range: (usize, usize)) -> Option<(usize, usize)> {
        let mut start = None;
        let mut end = None;
        for ((s, e), line) in self.lines.iter().zip(1_usize..) {
            if start.is_none() && range.0 >= *s && range.0 < *e {
                start = Some(line);
            }
            if end.is_none() && range.1 >= *s && range.1 < *e {
                end = Some(line);
            }
        }

        match (start, end) {
            (Some(s), Some(e)) => Some((s, e)),
            _ => None,
        }
    }

    #[inline]
    fn display_line(&self, lineno: usize, inline_pos: usize, len: usize) {
        let len = if len == 0 { 1 } else { len };

        eprintln!("{} | {}", lineno, self.get_line(lineno)); // 出错的行
        let width = format!("{}", lineno).len(); // 行号数字的长度
        eprintln!(
            // 箭头, 指向出错的部分.
            "{} | {}{}",
            " ".repeat(width),
            " ".repeat(inline_pos),
            "^".repeat(len)
        );
    }
    #[inline]
    fn get_line(&self, lineno: usize) -> &str {
        let (start, end) = self.lines[lineno - 1];
        &self.source[start..end]
    }
}
