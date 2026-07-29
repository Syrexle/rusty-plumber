from pathlib import Path
from PIL import Image, ImageDraw, ImageFont

ROOT = Path(__file__).resolve().parents[1]
ASSETS = ROOT / "assets" / "original"
OUT = ROOT / "assets" / "og-banner.png"
OUT_DIST = ROOT / "dist" / "assets" / "og-banner.png"
W, H = 240, 126
SCALE = 5

NES = [
    (15, 56, 15), (48, 98, 48), (93, 148, 76), (156, 207, 94),
    (12, 18, 44), (28, 44, 88), (63, 85, 134), (116, 144, 184),
    (32, 32, 32), (66, 66, 66), (116, 116, 116), (188, 188, 188),
    (252, 252, 252), (244, 184, 72), (196, 72, 36), (112, 36, 32),
    (232, 72, 64), (255, 241, 166), (84, 50, 32), (132, 76, 44),
    (32, 180, 200), (72, 220, 240), (126, 37, 83), (174, 74, 112),
]

def font(size, bold=True):
    path = "/System/Library/Fonts/Supplemental/Arial Bold.ttf" if bold else "/System/Library/Fonts/Supplemental/Arial.ttf"
    return ImageFont.truetype(path, size)

def paste_asset(dst, filename, xy=(0, 0), size=None, flip=False):
    img = Image.open(ASSETS / filename).convert("RGBA")
    if flip:
        img = img.transpose(Image.Transpose.FLIP_LEFT_RIGHT)
    if size is not None:
        img = img.resize(size, Image.Resampling.NEAREST)
    dst.alpha_composite(img, xy)

img = Image.new("RGBA", (W, H), (12, 18, 44, 255))
d = ImageDraw.Draw(img)

# industrial blue tile backdrop
for y in range(0, H, 16):
    for x in range(0, W, 32):
        c = (28, 44, 88, 255) if ((x//32 + y//16) % 2 == 0) else (34, 53, 101, 255)
        d.rectangle([x, y, x+31, y+15], fill=c)
        d.line([x, y+15, x+31, y+15], fill=(12, 18, 44, 255))
        d.line([x+31, y, x+31, y+15], fill=(12, 18, 44, 255))

# NES-style pixel clouds/spark dashes
for x,y,w in [(9,13,12),(42,25,8),(89,12,14),(143,26,10),(207,16,12),(188,42,6)]:
    d.rectangle([x,y,x+w,y+1], fill=(116,144,184,255))

# big moon/pipe vignette behind title
d.rectangle([4,4,W-5,H-5], outline=(252,252,252,255), width=1)
d.rectangle([7,7,W-8,H-8], outline=(66,66,66,255), width=1)

# pipe shapes
for px, py, ph in [(18, 67, 34), (199, 60, 41)]:
    d.rectangle([px, py, px+18, py+ph], fill=(48,98,48,255), outline=(15,56,15,255))
    d.rectangle([px-3, py, px+21, py+8], fill=(93,148,76,255), outline=(15,56,15,255))
    d.line([px+4, py+10, px+4, py+ph], fill=(156,207,94,255))

# ground/platforms
for y in [100,108,116]:
    col = (66,66,66,255) if y==100 else ((32,32,32,255) if y==116 else (116,116,116,255))
    d.rectangle([0,y,W,y+7], fill=col)
d.rectangle([0,96,W,99], fill=(188,188,188,255))
for x in range(0,W,16):
    d.rectangle([x,100,x+15,115], outline=(32,32,32,255))

# floating platform accents
for x,y,wid in [(38,80,44),(145,76,50),(185,88,32)]:
    d.rectangle([x,y,x+wid,y+5], fill=(116,116,116,255), outline=(32,32,32,255))
    d.rectangle([x,y+6,x+wid,y+9], fill=(32,32,32,255))

# title panel and title
panel = (9, 16, 148, 64)
d.rectangle(panel, fill=(12,18,44,230), outline=(252,252,252,255))
d.rectangle([panel[0]+2,panel[1]+2,panel[2]-2,panel[3]-2], outline=(196,72,36,255))
# shadow then text
f1 = font(21, True)
f2 = font(18, True)
d.text((16,19), "RUSTY", font=f1, fill=(32,32,32,255))
d.text((14,17), "RUSTY", font=f1, fill=(255,241,166,255))
d.text((16,40), "PLUMBER", font=f2, fill=(32,32,32,255))
d.text((14,38), "PLUMBER", font=f2, fill=(252,252,252,255))
small = font(6, True)
d.text((16,59), "8-BIT PIPE PLATFORMER", font=small, fill=(244,184,72,255))

# sprites from project assets
paste_asset(img, "player_idle.png", xy=(32, 62), size=(38, 38))
paste_asset(img, "player_idle.png", xy=(172, 45), size=(34, 34), flip=True)
paste_asset(img, "shop.png", xy=(73, 74), size=(32, 28))
paste_asset(img, "goal.png", xy=(194, 48), size=(38, 58))
# crop the oversized goal into a right-edge goal element instead of letting it dominate
ImageDraw.Draw(img).rectangle([226, 36, W, 118], fill=(12,18,44,255))
for i,(x,y) in enumerate([(107,83),(126,72),(151,62),(204,78),(54,68),(223,48)]):
    paste_asset(img, "apple.png", xy=(x, y), size=(20, 20))

# NES cartridge/HUD language
d.rectangle([158,14,225,33], fill=(12,18,44,220), outline=(188,188,188,255))
d.text((162,18), "PRESS START", font=small, fill=(252,252,252,255))
d.text((162,26), "syrexle.github.io", font=small, fill=(116,144,184,255))

# upscale crisply
out = img.convert("RGB").resize((W*SCALE, H*SCALE), Image.Resampling.NEAREST)

# subtle CRT scanlines after upscale, thin enough not to hurt OG readability
scan = Image.new("RGBA", out.size, (0, 0, 0, 0))
sd = ImageDraw.Draw(scan)
for y in range(0, out.size[1], 20):
    sd.rectangle([0, y, out.size[0], y + 1], fill=(0, 0, 0, 38))
out = Image.alpha_composite(out.convert("RGBA"), scan).convert("RGB")

# add exact file dimensions metadata by saving PNG
OUT.parent.mkdir(parents=True, exist_ok=True)
out.save(OUT)
OUT_DIST.parent.mkdir(parents=True, exist_ok=True)
out.save(OUT_DIST)
print(f"wrote {OUT} and {OUT_DIST} ({out.size[0]}x{out.size[1]})")
