use std::any::type_name;

/// 获取类型名称
/// 
/// Examples
/// 
/// ```
/// struct Demo {};
/// 
/// let t = type_of(Demo{})
/// 
/// 
pub fn type_of<T>(_: T) -> &'static str {
    type_name::<T>()
}