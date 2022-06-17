# Journal

Simple Journal App to track your day from the terminal.

Edit your notes with your editor of choice.

# Features

- Simple File Structure
- It's all plain markdown files
- Don't leave the terminal

# File Structure
The Root of the Journal contains a ```Notes``` Folder and every journal entry will be stored in subfolders like ```YEAR/MONTH/DAY/```.

The ```Journal``` Folder will be placed in the Documents Folder or ```$XDG_CONFIG_HOME``` by default. The location the folder will be placed in can be controlled by the ```$JOURNAL_DIR``` Environment Variable.
# Dependencies

- A Text Editor (nano, vim, micro): The Variable $EDITOR will be used
- Glow (Optional): Makes the Markdown look good