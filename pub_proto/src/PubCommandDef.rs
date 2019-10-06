
use phf::phf_map;
pub static COMMAND_MAP: phf::Map<&'static str, u32> = phf_map!{
    "create_account" => 0x233,
};
