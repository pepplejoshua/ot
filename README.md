# Catching up

When people code together on Replit, everyone's code needs to be in sync. You have to see the same code as I do even though we're typing on different computers. The challenge is making sure we don't end up with a jumbled mess of text while we type together.

So in order to keep everyone's code in sync, Replit uses a method called Operational Transformations, or OT.

Think about OT like this: when you type, you can either insert text, delete text, or move your cursor to a new position (this is called skip in OT land). These actions are called operations, and they transform your document!

More concretely, you can look at an Operational Transformation as a function that takes in a document, a position within that document (like where your cursor is), and then either modifies the document at that position or skips to a new position

## Some examples:

1. ```
   Input document: ""
   Starting cursor position: 0
   Operation: {"op": "insert", "chars": "Hello, human!"}
   Output document: "Hello, human!"
   Ending cursor position: 13
   ```

2. ```
   Input document: "What is up?"
   Starting cursor position: 7
   Operation: {"op": "delete", "count": 3}
   Output document: "What is?"
   Ending cursor position: 7
   ```

   > Watch out: delete operations are applied forward while keeping the cursor in place. Crazy, we know.

3. ```
   Input document: "Nice!"
   Starting cursor position: 0
   Operation[1]: {"op": "skip", "count": 4}
   Operation[2]: {"op": "insert", "chars": " day"}
   Output document: "Nice day!"
   Ending cursor position: 8
   ```

   > As you can see, this last example applies two transformations in a row.
