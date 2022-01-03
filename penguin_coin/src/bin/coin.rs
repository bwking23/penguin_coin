use penguin_coin::chapter_one;
use thiserror::Error;

#[derive(Error, Debug)]
enum CoinErrors {
    #[error(transparent)]
    FiniteSetError(#[from] chapter_one::FiniteSetError),
}

type Result<T> = std::result::Result<T, CoinErrors>;

fn main() -> Result<()> {
    let x = chapter_one::FieldElement::new(4, 3)?;
    println!("{:#?}", x);
    Ok(())
}
