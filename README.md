# control_scrapper

## Presentation

This is a simple project that will get a random image from a [Faden Friday](https://controlgame.com/faden-friday-march-2023/) that has a 16/9 ratio.
By default it will store the image wallpaper.jpeg in a subdirectory of your picture directory named ControlScrapperRes.
And store which image and when in a file image_log.log in a subdirectory .control_scrapper of your home directory.

## Configuration

You can change the target files for the logs and the images by creating a file control_scrapper.conf in the .control_scrapper directory.
The first line is the path to the logging file and the second is the path for where the image will be stored.

Currently, every directory in the paths needs to be created before your first start, and the logging file also needs to be made beforehand.
