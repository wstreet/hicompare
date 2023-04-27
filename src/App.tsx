import { invoke } from "@tauri-apps/api/tauri";
import CodeMirrorMerge from 'react-codemirror-merge';
import { EditorView } from 'codemirror';

const Original = CodeMirrorMerge.Original;
const Modified = CodeMirrorMerge.Modified;


function App() {
  return (
    <CodeMirrorMerge>
      <Original />
      <Modified
        extensions={[EditorView.editable.of(true)]}
      />
    </CodeMirrorMerge>
  );
}

export default App;
