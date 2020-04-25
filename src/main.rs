use structopt::StructOpt;

use radix_trie;

#[derive(Debug)]
struct INode<'a> { // Index Node.
    addr: [u8; 10],
    next: &'a INode<'a>,
}

// DISK LAYOUT

const BLOCK_SIZE: u16 = 2 << 11; // For simplicity, like a page.
const BLOCK_NUM: u8 = 2 << 7; // 256 is good enough for now.
const BLOCK_FILE: &'static str = "block.disk";

#[derive(StructOpt, Debug)]
/// Manage a dummy file system that uses an existing file as block device.
enum Cli {
    #[structopt(verbatim_doc_comment)]
    /// Create a file on the block device
    Create {
        path: String,
    },
    #[structopt(verbatim_doc_comment)]
    /// Read all contents of the file
    Read {
        path: String,
    },
    #[structopt(verbatim_doc_comment)]
    /// Append to the end of a file
    Write {
        path: String,
    },
    #[structopt(verbatim_doc_comment)]
    /// The path of the file or directory to remove from the block
    Remove {
        path: String,
    },
    #[structopt(verbatim_doc_comment)]
    /// List directory contents
    List {
        path: String,
    },
}

fn main() {
    let inodes: radix_trie::Trie<&str, INode> = radix_trie::Trie::new();
    let mut free_bitmap = [true; BLOCK_NUM as usize];

    match Cli::from_args() {
        Cli::List { path } => {
            let res = inodes.get(path.as_str());
            println!("Found files:\n {:?}", res)
        }
        _ => ()
    }
}
