#[derive(Debug)]
pub enum FormatError {
    UnexpectedFormat,
    WrongLenght(usize),
}

#[derive(Debug)]
pub struct LineError {
    pub n: usize,
    pub error: FormatError,
}

#[derive(Debug)]
pub enum Error {
    EmptyInput,
    WrongLine(usize, FormatError),
    SameNumberOfBit,
    Unexpected,
}

const SEQ_LEN: usize = 12;
type BinSeq = [bool; SEQ_LEN];
type OccSeq = [i64; SEQ_LEN];

fn line_as_value_vec(line: String) -> Result<OccSeq, FormatError> {
    let bin_seq: BinSeq = line
        .chars()
        .map(|c| match c {
            '0' => Ok(false),
            '1' => Ok(true),
            _ => Err(FormatError::UnexpectedFormat),
        })
        .collect::<Result<Vec<bool>, FormatError>>()?
        .try_into()
        .map_err(|e: Vec<bool>| FormatError::WrongLenght(e.len()))?;

    let occ_seq = bin_seq.map(|c| match c {
        false => -1i64,
        true => 1i64,
    });
    Ok(occ_seq)
}

pub fn first_part(lines: Vec<String>) -> Result<usize, Error> {
    let mut as_diff_occ = lines
        .into_iter()
        .enumerate()
        .map(|(n, line)| line_as_value_vec(line).map_err(|e| Error::WrongLine(n, e)))
        .collect::<Result<Vec<_>, Error>>()?
        .into_iter();

    let bytes: BinSeq = match as_diff_occ.next() {
        None => return Err(Error::EmptyInput),
        Some(diff_occ_counter) => {
            let occ_diff_vec = as_diff_occ.fold(diff_occ_counter, |acc, x| acc.zip(x).map(|(acc, x)| acc + x));

            occ_diff_vec
                .into_iter()
                .map(|x| match x {
                    x if x < 0 => Ok(false),
                    0 => Err(Error::SameNumberOfBit),
                    _ => Ok(true),
                })
                .collect::<Result<Vec<_>, Error>>()?
                .try_into()
                .map_err(|_| Error::Unexpected)
        }
    }?;

    let mut gamma = 0;
    let mut epsilon = 0;
    for i in 0..SEQ_LEN {
        if bytes[SEQ_LEN - 1 - i] {
            gamma += 1 << i;
        } else {
            epsilon += 1 << i;
        }
    }

    Ok(gamma * epsilon)
}
