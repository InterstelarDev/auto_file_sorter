use std::env;
use std::fs;
use std::fs::rename;
use std::path::Path;
// run in the specific directory you want to sort
//Im gonna add comments here to remmember what I've learnt 
fn main() {
    //reads args
    //the first argument is mapped to &str and then unwrapped and passed to target_dir
    //entries willread the target dir, if it cant it will show an error
    let args: Vec<String> = env::args().collect();
    let target_dir = args.get(1).map(|s| s.as_str()).unwrap_or(".");
    let entries = fs::read_dir(target_dir).expect("Error: Could not read directory");

    //for loop, for each entry in entries
    //var entry will be each entry, if it cannot read an entry it will give an error
    //path is the path to each entry
    //if the path is a file
    //take the extension of it and change type to str, unwrap and make it lowercase for consistency
    //then pass that to a match which will then see which folders need to be created
    //then all of the folders are created using fs
    //then the filename will be stroed in filename
    //and filename_str will unwrap filename to a string, or it will pass an empty string if it cant
    //exception for this specific project file so it doesnt sort everyhing here
    //creates a destination path for each file
    //uses rename to move all the files
    for entry in entries {
        let entry = entry.expect("Error, Could not read entry");
        let path = entry.path();

        if path.is_file() {
            if let Some(ext ) = path.extension() {
                let ext_str = ext.to_str().unwrap().to_lowercase();
                
                let folder = match ext_str.as_str() {
                    "jpg" | "jpeg" | "png" | "gif" | "svg" | "webp" => "Images",
                    "mp4" | "mov" | "avi" | "mkv" | "wmv" => "Video",
                    "mp3" | "wav" | "flac" | "m4a" | "ogg" => "Music",
                    "pdf" | "doc" | "docx" | "txt" | "rtf" | "odt" => "Documents",
                    "ppt" | "pptx" | "xls" | "xlsx" | "csv" => "Data",
                    "zip" | "rar" | "7z" | "tar" | "gz" | "iso" => "Archives",
                    "rs" | "py" | "js" | "html" | "css" | "cpp" | "c" | "json" => "Code",
                    "sh" | "bat" | "exe" | "msi" => "Executables",
                    _ => "Others", 
};
                let folder_path = Path::new(target_dir).join(folder);
                fs::create_dir_all(&folder_path).expect("Error, Failed to create folder");

                let file_name = path.file_name().expect("Error, Could not get filename");
                let file_name_str = file_name.to_str().unwrap_or("");

                if file_name_str.starts_with('.') || file_name_str == "Cargo.toml" || file_name_str == "afs" {
                    continue; 
                }
                let dest_path = folder_path .join(file_name);
                
                rename(&path, dest_path).expect("Error, Failed to move file")
            }
        }

    }
}
