use store_num::entry as entry_store_num;
use store_num::ID as PROGRAM_ID_STORE_NUM;
const PROGRAM_NAME_STORE_NUM: &str = "store_num";
use fuzz_instructions::store_num_fuzz_instructions::FuzzInstruction as FuzzInstruction_store_num;
use trident_client::fuzzing::*;
mod accounts_snapshots;
mod fuzz_instructions;

pub type FuzzInstruction = FuzzInstruction_store_num;

struct MyFuzzData;

impl FuzzDataBuilder<FuzzInstruction> for MyFuzzData {}

fn main() {
    loop {
        fuzz_trident!(fuzz_ix: FuzzInstruction, |fuzz_data: MyFuzzData| {

            // Specify programs you want to include in genesis
            // Programs without an `entry_fn`` will be searched for within `trident-genesis` folder.
            // `entry_fn`` example: processor!(convert_entry!(program_entry))
            let fuzzing_program1 = FuzzingProgram::new(PROGRAM_NAME_STORE_NUM,&PROGRAM_ID_STORE_NUM,processor!(convert_entry!(entry_store_num)));

            let mut client =
                ProgramTestClientBlocking::new(&[fuzzing_program1])
                    .unwrap();

            // fill Program ID of program you are going to call
            let _ = fuzz_data.run_with_runtime(PROGRAM_ID_STORE_NUM, &mut client);
        });
    }
}
