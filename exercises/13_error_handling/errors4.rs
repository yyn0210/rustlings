// errors4.rs
// Execute `rustlings hint errors4` or use the `hint` watch subcommand for a hint.

#[derive(PartialEq, Debug)]
struct PositiveNonzeroInteger(u64);

#[derive(PartialEq, Debug)]
enum CreationError {
    Negative,
    Zero,
}

fn main() {
    // You can add some code here to test the PositiveNonzeroInteger::new function if needed
}

impl PositiveNonzeroInteger {
    fn new(value: i64) -> Result<PositiveNonzeroInteger, CreationError> {
        // 检查 value 是否为负数
        if value < 0 {
            return Err(CreationError::Negative);
        }

        // 检查 value 是否为零
        if value == 0 {
            return Err(CreationError::Zero);
        }

        // 返回正确的结果
        Ok(PositiveNonzeroInteger(value as u64))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_creation() {
        assert!(PositiveNonzeroInteger::new(10).is_ok());
        assert_eq!(
            Err(CreationError::Negative),
            PositiveNonzeroInteger::new(-10)
        );
        assert_eq!(Err(CreationError::Zero), PositiveNonzeroInteger::new(0));
    }
}