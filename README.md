# Win95-KeyTool

[![Build](https://github.com/nandolawson/Win95-KeyTool/actions/workflows/build.yml/badge.svg)](https://github.com/nandolawson/Win95-KeyTool/actions/workflows/build.yml)

This software is a cross-plattform generator and validator for Windows 95 keys written in Rust. As it turned out, however, it supposedly works for all Microsoft products that require a 10-digit product key in the following format: _`XXX-XXXXXXX`_

It is only sporadically developed as it is solely a learning project for me and does not have particularly significant practical use.
>⚠️ **Note**: Win95 Key Tool and I are in no way associated with Microsoft or their products. Only [publicly available information](https://en.wikipedia.org/wiki/Product_key#Examples) from the internet has been used. It neither bypasses effective copy measures nor constitutes a "crack." The purpose of this repository, from my perspective, is solely to gain experience in programming and enhance my skills and knowledge.

## Compatible Software

- Access ADI 95
- Hell Bender
- Office 7.0b
- Office Professional 95
- Plus! 95
- Plus! 98
- Return to Arcade
- Windows 95
- Windows CE Toolkit for Visual Basic 5
- Windows CE Toolkit for Visual C++ 5
- Windows NT 4.0 Server
- Windows NT 4.0 Workstation
- Visual Basic Standard 4.0
- Visual SourceSafe 4.0

## Usage

To use the software, it needs to be launched via a terminal. There are two options:

### Generate

Launch the software in a terminal window with _generate_ as additional argument. In a fraction of a second, a key will be generated that meets all the requirements for activating Windows 95. The command in the Windows command prompt would look something like this:

```bat
.\<PATH_TO_EXECUTABLE\win95-keytool.exe generate
```

### Validate

To check the validity of a key, you can use it as an argument. This can be done in the command prompt like this:

```bat
.\<PATH_TO_EXECUTABLE\win95-keytool.exe 012-3456789
```

## Contributing

Anyone who wants to contribute is more than welcome to do so. I would be delighted to learn from the contributions of other users. If you find a bug or have a feature in mind that you think would be useful, please feel free to create an issue or a pull request here on GitHub.
If you decide to fork this project, please make sure to adhere to the [license](https://github.com/nandolawson/Win95-KeyTool/blob/master/LICENSE). Your involvement and feedback are highly appreciated!
