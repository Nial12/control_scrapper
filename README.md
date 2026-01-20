# control_scrapper

## Presentation

This is a simple project that will obtain an image from a list in a txt file, there is an example file named images.txt in the git.
You can also use the configuration to change the obtention of the image to parsing a website in the format base_url+date with date between the current date and 1st of February 2023.
By default it will store the image wallpaper.jpeg in a subdirectory of your picture directory named ControlScrapperRes.
And store which image and when in a file image_log.log in a subdirectory .control_scrapper of your home directory.

## Configuration

There is a default configuration the git.
You can change the target files for the logs and the images by creating a file control_scrapper_d.conf in the .control_scrapper directory.
The first line is the path to the logging file and the second is the path for where the image will be stored.

Currently, every directory in the paths needs to be created before your first start, and the logging file also needs to be made beforehand.
If you get the image from a txt list, it will comment lines that links to images with the wrong ratio.
