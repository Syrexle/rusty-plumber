#!/usr/bin/env python3
from pathlib import Path
from PIL import Image, ImageDraw

ROOT = Path(__file__).resolve().parents[1]
OUT = ROOT / "assets" / "original"
OUT.mkdir(parents=True, exist_ok=True)

SCALE = 4

def img(name, size, bg=(0,0,0,0)):
    im = Image.new("RGBA", size, bg)
    return im, ImageDraw.Draw(im)

def save(im, name):
    im.save(OUT / name)

# Background: original tiled dusk/pipes scene.
im, d = img("background.png", (320, 180), (36, 28, 86, 255))
for y, col in [(0,(30,27,74,255)), (45,(66,52,128,255)), (105,(238,98,150,255)), (150,(255,188,92,255))]:
    d.rectangle([0,y,320,y+45], fill=col)
for x,y,r in [(35,35,17),(92,55,11),(178,28,15),(262,70,13)]:
    d.ellipse([x-r,y-r,x+r,y+r], fill=(255,232,143,120))
for x in range(-40,340,58):
    d.rectangle([x,132,x+18,180], fill=(36,165,102,190))
    d.rectangle([x-6,126,x+24,137], fill=(52,207,128,220))
for x in range(0,320,16):
    d.rectangle([x,168,x+16,180], fill=(30,35,63,200))
save(im, "background.png")

# Player: plumber/frog-ish hero.
im, d = img("player_idle.png", (32,32))
d.rectangle([10,9,21,24], fill=(52,212,106,255))
d.rectangle([8,12,23,20], fill=(45,184,92,255))
d.rectangle([7,5,24,11], fill=(230,45,62,255))
d.rectangle([10,2,21,6], fill=(246,78,83,255))
d.rectangle([12,12,14,14], fill=(255,255,255,255)); d.point((13,13), fill=(20,20,30,255))
d.rectangle([18,12,20,14], fill=(255,255,255,255)); d.point((19,13), fill=(20,20,30,255))
d.rectangle([12,24,15,29], fill=(33,92,205,255)); d.rectangle([17,24,20,29], fill=(33,92,205,255))
d.rectangle([8,29,15,31], fill=(78,48,35,255)); d.rectangle([17,29,24,31], fill=(78,48,35,255))
save(im, "player_idle.png")

im, d = img("terrain_tile.png", (32,32))
d.rectangle([0,0,31,31], fill=(142,84,45,255))
for y in range(0,32,8): d.line([0,y,31,y], fill=(96,54,37,255))
for x in range(0,32,8): d.line([x,0,x,31], fill=(96,54,37,255))
d.rectangle([0,0,31,6], fill=(75,202,90,255))
d.rectangle([0,6,31,9], fill=(40,134,69,255))
save(im, "terrain_tile.png")

im, d = img("terrain_grass.png", (32,10))
d.rectangle([0,0,31,9], fill=(52,185,76,255))
for x in range(0,32,4): d.polygon([(x,9),(x+2,1),(x+4,9)], fill=(91,234,105,255))
save(im, "terrain_grass.png")

im, d = img("hazard_spike.png", (32,18))
d.rectangle([0,14,31,17], fill=(65,65,85,255))
for x in range(0,32,8): d.polygon([(x,14),(x+4,1),(x+8,14)], fill=(230,230,238,255)); d.line([(x+4,1),(x+6,14)], fill=(150,150,170,255))
save(im, "hazard_spike.png")

im, d = img("apple.png", (24,24))
d.ellipse([5,6,18,20], fill=(235,42,65,255)); d.ellipse([12,7,20,20], fill=(255,72,68,255))
d.rectangle([11,3,13,8], fill=(96,58,29,255)); d.ellipse([13,3,20,8], fill=(80,210,96,255))
d.rectangle([8,9,10,11], fill=(255,184,170,255))
save(im, "apple.png")

im, d = img("enemy_rock.png", (32,32))
d.ellipse([4,7,27,28], fill=(139,92,72,255))
d.polygon([(7,13),(12,6),(17,14)], fill=(195,154,95,255)); d.polygon([(17,14),(23,6),(26,16)], fill=(195,154,95,255))
d.rectangle([10,16,13,19], fill=(255,245,160,255)); d.rectangle([20,16,23,19], fill=(255,245,160,255))
d.rectangle([11,17,12,18], fill=(30,25,30,255)); d.rectangle([21,17,22,18], fill=(30,25,30,255))
d.rectangle([12,24,24,26], fill=(72,45,42,255))
save(im, "enemy_rock.png")

im, d = img("checkpoint.png", (32,48))
d.rectangle([5,5,8,45], fill=(82,54,44,255))
d.polygon([(8,6),(27,12),(8,20)], fill=(252,203,66,255)); d.rectangle([4,43,15,47], fill=(106,72,45,255))
save(im, "checkpoint.png")

im, d = img("shop.png", (40,36))
d.rectangle([5,14,35,33], fill=(170,88,52,255)); d.rectangle([8,18,18,33], fill=(94,52,36,255)); d.rectangle([22,19,31,26], fill=(255,225,120,255))
d.rectangle([2,8,38,15], fill=(250,84,126,255)); d.rectangle([5,4,35,9], fill=(255,204,87,255))
d.ellipse([14,0,25,10], fill=(255,168,196,255)); d.rectangle([18,6,21,10], fill=(130,66,88,255))
save(im, "shop.png")

im, d = img("goal.png", (40,56))
d.rectangle([18,8,22,54], fill=(225,225,230,255))
d.polygon([(22,8),(38,14),(22,22)], fill=(77,235,147,255)); d.rectangle([10,50,30,55], fill=(94,65,45,255))
for r,c in [(18,(255,234,92,255)),(11,(255,140,74,255)),(5,(255,255,255,255))]: d.ellipse([20-r,30-r,20+r,30+r], outline=c, width=2)
save(im, "goal.png")

im, d = img("confetti.png", (16,16))
colors=[(255,78,96,255),(255,222,70,255),(83,231,130,255),(93,188,255,255),(219,118,255,255)]
for i,c in enumerate(colors):
    x=(i*3+2)%14; y=(i*5+1)%14
    d.rectangle([x,y,x+2,y+5], fill=c)
save(im, "confetti.png")

im, d = img("dust.png", (32,12))
d.ellipse([1,4,12,10], fill=(255,255,255,70)); d.ellipse([9,2,25,11], fill=(255,255,255,55)); d.ellipse([21,5,31,10], fill=(255,255,255,50))
save(im, "dust.png")

print(f"Wrote {len(list(OUT.glob('*.png')))} original assets to {OUT}")
