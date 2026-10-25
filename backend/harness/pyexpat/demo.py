import pyexpat, xml.etree.ElementTree as ET  # run: PYTHONPATH=<out-dir> python3 demo.py
print("pyexpat:", pyexpat.__file__, "|", pyexpat.EXPAT_VERSION)
root = ET.fromstring('<inventory owner="argon"><item id="1">bolt</item><item id="2">nut &amp; washer</item></inventory>')
print(ET.tostring(root, encoding="unicode"), [(i.get("id"), i.text) for i in root])
p = pyexpat.ParserCreate(); p.StartElementHandler = lambda n, a: print("start", n, a); p.Parse(b"<a x='1'><b/></a>", True)
