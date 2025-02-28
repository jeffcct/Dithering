from PIL import Image
import numpy as np

def find_closest(colors, oldpixel):
    differences = [abs(color - oldpixel) for color in colors]
    index = np.argmin(differences)
    return colors[index]


def add(x, y):
    return x + y

def subtract(x,y):
    return x - y

im = Image.open("data/flower2.jpeg")
grey = im.convert("L")
im_grey_vec = np.array(grey).astype(np.int16)
im_vec = np.array(im).astype(np.int16)

colors = [0, 128, 255]

height, width = im_grey_vec.shape

arr = im_vec.copy()

for y in range(height):
    for x in range(width):
        for c in range(3):
            oldpixel = im_vec[y,x,c]
            newpixel = find_closest(colors, oldpixel)
            im_vec[y,x,c] = newpixel
            quant_error = oldpixel - newpixel

            if x + 1 < width:
                im_vec[y, x+1,c] += (quant_error * 7) / 16

            if x > 0 and y + 1 < height:
                im_vec[y+1, x-1,c] += (quant_error * 3) / 16
                
            if y + 1 < height:
                im_vec[y+1, x,c] += (quant_error * 5) / 16

            if x + 1 < width and y + 1 < height:
                im_vec[y+1, x+1,c] += quant_error / 16

                
im_vec = np.clip(im_vec, 0, 255).astype(np.uint8)

image = Image.fromarray(im_vec, "RGB")
image.show()

arr2 = arr.copy()

for y in range(height):
    for x in range(width):
        for c in range(3):
            oldpixel = arr[y,x,c]
            newpixel = find_closest(colors, oldpixel)
            arr[y,x,c] = newpixel

arr = np.clip(arr, 0, 255).astype(np.uint8)

image = Image.fromarray(arr, "RGB")
# image.show()



# def find_closest_rgb(colors: list[np.ndarray], pixel: np.ndarray):
#     differences = [sum(abs(color - pixel)) for color in colors]
#     index = np.argmin(differences)
#     return colors[index]


# rgb_colors = [np.array([0,0,0]),
#               np.array([255, 0,0]), 
#               np.array([0, 255, 0]), 
#               np.array([0,0,255]),
#               np.array([0, 255, 255]),
#               np.array([255, 0, 255]),
#               np.array([255, 255, 0]),
#               ]

# for y in range(height):
#     for x in range(width):
#         oldpixel = arr2[y,x].copy()
#         newpixel = find_closest_rgb(rgb_colors, oldpixel)
#         arr2[y,x] = newpixel
#         quant_error = oldpixel - newpixel
#         #print(quant_error, oldpixel, newpixel)

#         if x + 1 < width:
#             arr2[y, x+1] += ((quant_error * 7) // 16).astype(np.int16)

#         if x > 0 and y + 1 < height:
#             arr2[y+1, x-1] += ((quant_error * 3) / 16).astype(np.int16)
            
#         if y + 1 < height:
#             arr2[y+1, x] += ((quant_error * 5) / 16).astype(np.int16)

#         if x + 1 < width and y + 1 < height:
#             arr2[y+1, x+1] += (quant_error / 16).astype(np.int16)


# arr2 = np.clip(arr2, 0, 255).astype(np.uint8)

# image = Image.fromarray(arr2, "RGB")
# image.show()