import os
os.chdir("./wasm")
os.system('cmd /c "wasm-pack build --target web"')

with open("./pkg/wiki_graph.js", 'r') as file:
    existing_content = file.read()

    # Combine the new text with the existing content
    new_content = "import { display_result } from \"../lib/wiki_graph.js\";\n" + existing_content

    # Write the combined content back to the file
    with open("./pkg/wiki_graph.js", 'w') as file:
        file.write(new_content)
        
if os.name == 'nt':   
    os. remove("./pkg/.gitignore")
    os.system('cmd /c  "robocopy  ./pkg ../web/src/pkg" /E /MOVE /NFL /NDL /NJH /NJS /nc /ns /np')
else:
    print("wasm package moving to web-editor not implemented on other platforms")