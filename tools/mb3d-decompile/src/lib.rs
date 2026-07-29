//! Library face of the decompiler, so its results can be asserted on.

pub mod abi;
pub mod exec;
pub mod expr;
pub mod extract;

/// Decompiles one `.m3f` to `place = expression` lines.
pub fn decompile_file(path: &std::path::Path) -> Result<Vec<String>, String> {
    let formula = extract::parse(path).ok_or("no [CODE] block")?;
    let result = exec::run(&formula.code);
    if let Some(reason) = result.bailed {
        return Err(reason);
    }
    Ok(exec::final_stores(&result.stores)
        .into_iter()
        .map(|(place, value)| format!("{place} = {}", expr::render(&value)))
        .collect())
}
