# OX
A simple number formatting convertor

---

### Usage
```bash
ox <VALUE> -f <FORMAT>
```

Default format is decimal
```bash
ox 255 -f 0x      # 0xff
ox 255 -f 0x -n   # ff
```

Different formats
```bash
ox 0b11111111     # 255
ox 0xff           # 255
ox 0o377          # 255
```
