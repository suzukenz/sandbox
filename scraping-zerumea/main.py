import re
import shutil
import time
import random

import requests
from bs4 import BeautifulSoup

imgRegex = re.compile(r".+/\d{1,3}-\d{4}_.*\.jpg")

htmlFiles = [
    "input/5-floor.htm",
    "input/6-floor.htm",
    "input/7-floor.htm",
    "input/8-floor.htm"
]

if __name__ == "__main__":

    for htmlFile in htmlFiles:
        with open(htmlFile) as f:
            soup = BeautifulSoup(f, "html.parser")
            images = soup.find_all(
                "img", src=imgRegex)

            for img in images:
                imgURL = img.get("src")
                fileName = imgURL.split("/")[-1]

                response = requests.get(imgURL, stream=True)
                if not response.ok:
                    print("error:" + imgURL)
                    continue

                with open("images/" + fileName, 'wb') as outFile:
                    try:
                        shutil.copyfileobj(response.raw, outFile)
                    except:
                        print("error:" + imgURL)

                time.sleep(random.randint(1, 3))
