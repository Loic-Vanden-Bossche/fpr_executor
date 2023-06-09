use clap::ValueEnum;

#[derive(ValueEnum, Debug, Clone, Copy)]
#[clap(rename_all = "kebab_case")]
pub enum ExecutorType {
    Python,
    Node,
    Binary,
}
