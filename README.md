# 1 File Organizer Application Instructions

### The Problem
When using a computer, we tend to download files to a single location, such as "./Downloads". This folder quickly becomes cluttered and challenging to navigate, making it difficult to find the files we need. As the folder grows, it becomes increasingly time-consuming and tedious to organize the files manually.

### Solution
This File Organizer application helps organize files by moving them to folders based on their file extensions. To use the application, you need to have a configuration file named "file_map-config.json" that maps file extensions to folder names. 

You will receive this file when you install the program. It will be located in the root of your installation folder, such as C:\Program Files\FileOrganizer.

# 2 Installing and setting configuration
To use the File Organizer application, follow these steps:

1. Download the installer from the following link: _.
2. Install the application.
3. Run the application.
4. While you can use it straight away, it is recommended to modify the configuration file to your liking.
5. Configure the folder structure by opening the side panel.
6. You can use any text editor for this.

![Untitled scene (2)](https://user-images.githubusercontent.com/83360104/222018052-c13f77d1-1816-4f5e-be06-a11e1a683a14.png)  
![image](https://user-images.githubusercontent.com/83360104/222017811-64f6af5e-66f5-4bba-809b-5475a0d52d33.png)

---

## Configuration starting point:
An example of how the configuration file should look like:

```json
{
  "Docs": ["docx", "odt", "rtf", "txt", "ods", "pptx", "dotx", "doc", "ppt"],
  "Photos": ["jpg", "png", "jpeg", "ico", "jfif", "psd", "gif"],
  "Music": ["mp3", "wav", "aac", "ogg", "aif", "m4a"],
  "Installers": ["exe", "msi"],
  "Zip files": ["zip", "rar", "gz", "7z"],
  "Music Production": ["flp", "fst", "als", "midi", "mid"],
  "Video": ["mp4", "gif", "mpeg", "mkv"],
  "Excel": ["xlsx", "csv", "xlsm", "xls"],
  "Sound Banks & Presets": ["fxp", "nmsv", "spf", "fxb"],
  "PDF": ["pdf"],
  "Misc": []
}
```
> # IMPORTANT! Do not remove "Misc": [].
> - If a file extension is not listed in the configuration file, it will be moved to a folder named "Misc". 
> - If "Misc" does not exist, the remaining files will not be moved.


### You can edit fields, delete fields, and add new fields to your liking. 

```json
{
  "Folder Name 1": ["extension1", "extension2", "extension3"],
  "Folder Name 2": ["extension4", "extension5", "extension6"]
}
```

1. Indicates that any file with extension1, extension2, or extension3 will be moved to the "Folder Name 1" folder, 
2. Any file with extension4, extension5, or extension6 will be moved to the "Folder Name 2" folder.

Similarly, the part `"Photos": ["jpg", "png", "jpeg", "ico", "jfif", "psd", "gif"]` means that any file with any of the following extensions: `["jpg", "png", "jpeg", "ico", "jfif", "psd", "gif"]` will be moved to the "Photos" folder."

---

# 3 Usage

## NOTE
When you use the File Organizer application, please note that you may experience a brief freeze that could feel like a crash. This is a result of the backup process, which involves compressing all files into a single zip file before moving them to the correct folders based on your configuration file.

During the backup process, the application zips all files, which can take some time depending on the size of your files. Once the compression is complete, the application moves the zip file to the backup folder and then moves the original files from their original location to the new folders based on your configuration file.

If you encounter any issues or have questions about the backup process, please let me know. 

## Run the program 
1. Navigate to folder you want to organize  
![image](https://user-images.githubusercontent.com/83360104/222024641-f6d2e2d9-6e9e-4514-9052-c9ee0660aea5.png)

2. Click and select the folder path + copy it (CTRL + C)  
![image](https://user-images.githubusercontent.com/83360104/222024679-4f05afbe-31af-49de-9a61-7787179d787d.png)

3. Open the application and paste the copied path to text field (CTRL + V) 
At that point the button become enabled 
![image](https://user-images.githubusercontent.com/83360104/222024728-b8350ce0-91ba-4d17-838a-00e1e60389be.png)

4. You're done  
![image](https://user-images.githubusercontent.com/83360104/222024924-7a08d61c-d8a7-4055-92e9-23fbbbe369db.png)



# 4 Important
This application is still in developement and therefore there might be bugs.
You do test this application with you'r own responsibility and therefore I'm not responsible for any lost files


- **Known Issues:** Not known issues
- **Reporting Bugs:** [Open Issue](https://github.com/juhamikael/FileOrganizer/issues)
- **Bug Fixes:** Fixes will be made available as soon as possible.
- **Contributing:** If you're interested in contributing to develop this application, you can contribute by forking the repository on GitHub and submitting a pull request with your changes. 

# 5 Release Notes:
## Version 0.9.0:
This is the first beta release of the application, so please expect some bugs and issues. If you encounter any problems, please report them by opening an issue on GitHub. I will review all bug reports and work to fix any issues as soon as possible.
- Initial beta release of the application 


# 6 Contact: 
Join my [Discord](https://discord.gg/cxp7EKw53w)


