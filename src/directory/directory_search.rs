use crate::directory::folder::Folder;

//need a struct to hold folders that will be queued 
pub struct DirectoryNode {
    folder : Folder,
    folder_queue : FolderQueue,
}

pub struct FolderQueue {
    //a raw priority queue that bases it's size off of Folder size
    size : i32,
    index : i32,
    folder_pri_queue : [Option<Folder>; 10], 
}
/*
impl FolderQueue {
    fn new() -> FolderQueue {
        FolderQueue {
            self.size : 0,
            self.index : 0,
            folder_pri_queue : [None; 10],
        }
    }

    fn add_folder(folder : Folder) -> bool {
        //if empty queue add at the front 
        if self.size == 0 {
            folder_pri_queue[self.index] = Folder;
            return true;
        }


        return false;
    }


}
*/
//this will populate some sizes into the folders by searching with BFS for 1 second per folder 
pub fn initial_search(){

}