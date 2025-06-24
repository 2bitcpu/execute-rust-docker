use bpaf::Bpaf;
use roxmltree_to_serde::{Config, NullValue, xml_string_to_json};

use std::io::Write;

type BoxError = Box<dyn std::error::Error>;

#[derive(Clone, Debug, Bpaf)]
#[bpaf(options, version)]
#[allow(dead_code)]
struct Args {
    #[bpaf(short, long)]
    input: std::path::PathBuf,
    #[bpaf(short, long)]
    output: std::path::PathBuf,
    #[bpaf(long, short('w'))]
    overwrite: bool,
    #[bpaf(short, long)]
    pretty: bool,
}

/*************  ✨ Windsurf Command ⭐  *************/
/// Convert an XML file to a JSON file.
///
/// This is a simple tool to convert an XML file to a JSON file. It reads the
/// XML file, converts it to a JSON object using the `roxmltree_to_serde` crate,
/// and then writes the JSON object to a file. The `pretty` option can be used
/// to format the JSON output.
///
/// # Errors
///
/// * If the input file does not exist
/// * If the input file is not a file
/// * If the input and output files are the same
/// * If the output file already exists and the `overwrite` option is not set
/*******  5c455fe7-28f9-4603-a67a-6af32327924d  *******/
fn main() -> Result<(), BoxError> {
    let args = args().run();
    validate_args(&args)?;

    let xml = std::fs::read_to_string(args.input)?;

    let conf = Config::new_with_custom_values(true, "", "", NullValue::Null);
    let json = xml_string_to_json(xml.clone(), &conf)?;

    let json_str = match args.pretty {
        true => serde_json::to_string_pretty(&json)?,
        false => serde_json::to_string(&json)?,
    };

    let mut file = std::fs::File::create(args.output)?;
    write!(file, "{}", json_str)?;
    file.flush()?;

    Ok(())
}

fn validate_args(args: &Args) -> Result<(), BoxError> {
    if !args.input.exists() {
        return Err(BoxError::from("Input file does not exist"));
    }
    if args.input.is_dir() {
        return Err(BoxError::from("Input must be a file"));
    }
    if args.input == args.output {
        return Err(BoxError::from("Input and output files are the same"));
    }
    if !args.overwrite && args.output.exists() {
        return Err(BoxError::from("Output file already exists"));
    } else if args.output.is_dir() {
        return Err(BoxError::from("Output must be a file"));
    }
    Ok(())
}
