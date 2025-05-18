## Usage

```
forensics.exe file1.rbxl ./dir/file2.rbxl ...
```

**Expect output like:**

```
file1.rbxl,628,639,2024M,2024M
./dir/file2.rbxl,176,203,2014L,2015M
```

You can also pass a list of files through _stdin_:

```
cat list.txt | forensics.exe
```

Where `list.txt` is like:

```
file1.rbxl
./dir/file2.rbxl
```

Supports `rbxl` or `rbxm` files saved with Studio versions 47 thru 😈😈 ~666 😈😈 (or whatever `VERSION_MAX` in `./src/constants.rs` says).

---

## How did I do it?

When you save an `rbxl` file on Rōblox Studio, many properties will be saved with each in-game instance. Even ones that you don't usually modify. For example, `Workspace` saves with:

- `AirDensity`,
- `AllowThirdPartySales`,
- `DecreaseMinimumPartDensityMode`,
- `StreamingIntegrityMode`,
- `TouchesUseCollisionGroups`,
- `TerrainWeldsFixed`,

... and so many more.

New properties get added to `Workspace` and other classes every version.

Even if you modify an older `rbxl` file, the new properties will also be added once you save again.

My program collects the class names and their respective properties and estimates based on which ones saved with your file.

The `./deserializer` module contains scripts heavily modified from [the `rbx_binary` crate](https://github.com/rojo-rbx/rbx-dom/tree/master/rbx_binary). The main difference is that it only takes property names (and not their values). Much faster.

---

### For `./src/const.rs`

Go to a page such as https://robloxapi.github.io/ref/class/ChatInputBarConfiguration.html and run the following JavaScript code in your devtools console, replacing the literal `{{}}` with a list of :

```js
var title = document.querySelector("#content h1").innerText.split("\n")[0];

function t(e) {
	return e.querySelector("h3").innerText.split("\n")[0];
}
function va(e) {
	q = e.querySelector(":last-child>.added");
	return q ? Number.parseInt(q.text) : 47;
}
function vr(e) {
	q = e.querySelector(":first-child>.removed");
	return q ? Number.parseInt(q.text) : NaN;
}

var ar = Array.from(document.querySelectorAll(".members-sections>section:has(.col-valuetype)")).map((e) => [
	t(e),
	va(e),
	vr(e),
]);
ar.sort((a, b) => b[1] - a[1]);
ar = ar.filter((e, i) => e[1] != 47 && e[2] == NaN));

var map_value = ar
	.filter((e, i) => i == 0 || ar[i - 1][1] != e[1])
	.map((e) => `        "${e[0]}" => ${e[1]},\n`)
	.join("");
copy(`"${title}" => phf_map! {\n${map_value}    },`);
```

This will be in your clipboard:

```
"ChatInputBarConfiguration" => phf_map! {
        "AutocompleteEnabled" => 588,
        "KeyboardKeyCode" => 574,
        "BackgroundColor3" => 554,
        "Enabled" => 514,
    },
```

Make sure to verify that your properties appear in actual live files. To get a list of sorted properties from a particular file, try this:

```sh
curl https://github.com/jarenm1/rbx-mcp/raw/46d5893c70bcfeff3cd94d8945c311f46441e264/output.rbxlx -L | grep -P "(?<=class=`")[^`"]+" -o | sort -u
```

### Other Files That I Consulted

- https://github.com/LionelBergen/RobloxHelloWorld/raw/8511247f9211e880f9d8cf3d9f9e1aaf8c125221/MyFirstRobloxApp.rbxlx
- https://github.com/TheEpicFace007/roblox-domino-pizza-api/raw/cf7550c8bebcf71b9765a30732d3e3d535097b18/roblox-domino-pizza-api.rbxlx
- https://github.com/jarenm1/rbx-mcp/raw/46d5893c70bcfeff3cd94d8945c311f46441e264/output.rbxlx
- https://github.com/nedisliker/nedisliker.github.io/raw/6238c80bc153f99e1343b432c8bc50f52b16edf6/krutoe/testplace.rbxlx
- https://github.com/rojo-rbx/rbx-test-files/raw/7519945aeda55b675290e09390a9c755d17a1bdc/places/all-instances-415/xml.rbxlx
- https://github.com/rojo-rbx/rbx-test-files/raw/7519945aeda55b675290e09390a9c755d17a1bdc/places/all-instances-454/xml.rbxlx
- https://github.com/rojo-rbx/rbx-test-files/raw/7519945aeda55b675290e09390a9c755d17a1bdc/places/baseplate-415/xml.rbxlx
- https://github.com/rojo-rbx/rbx-test-files/raw/7519945aeda55b675290e09390a9c755d17a1bdc/places/baseplate-454/xml.rbxlx
- https://github.com/rojo-rbx/rbx-test-files/raw/7519945aeda55b675290e09390a9c755d17a1bdc/places/baseplate-566/xml.rbxlx
- https://github.com/CodedNil/RobloxArchive/raw/94540ad05800cbc7ab69f6bfb4a43bacbfe247c7/Misc/Models/Animator.rbxmx
- https://github.com/Epix-Incorporated/Adonis-Plugins/raw/103a8b5a6e43d4564fdbcdc72c6b94e8faa7cc4d/Server/Server-AdonisChatAddition.rbxmx
- https://github.com/Epix-Incorporated/Adonis/raw/30496814e9f5f0d95e372ad680dc20905e12cb3f/MainModule/Client/UI/Default/Console.rbxmx
- https://github.com/Epix-Incorporated/Adonis/raw/30496814e9f5f0d95e372ad680dc20905e12cb3f/MainModule/Server/Dependencies/Assets/LinkedSword.rbxmx
- https://github.com/EveryoneDestroysTheWorld/turf-war/raw/ff34d42309e025ae39a1bb490a11c36d8e0c926b/src/Workspace/Terrain.rbxmx
- https://github.com/rojo-rbx/rbx-test-files/raw/7519945aeda55b675290e09390a9c755d17a1bdc/models/imagelabel-content/xml.rbxmx

---

This Python script copies (to your clipboard) the particular 'eras' that versions of Rōblox were built.

```py
import requests
import pyperclip
d={}
r=requests.get('https://setup.rbxcdn.com/DeployHistory.txt').text
def f(l):R=re.search(r'at (\d+)/\d+/(\d+) [\d:]+ [AP]M, file vers?ion: 0, (\d+)',l);m=int(R[1]);return (R[2]+('E' if m<5 else 'M' if m<9 else 'L'),int(R[3]))
for l in r.split('\r\n'):
 try:
  (era,v)=f(l)
  d.setdefault(era,v)
 except Exception:
  continue
pyperclip.copy('\n'.join(f'    ("{k}", {e}),' for k,e in reversed(d.items())))
```
