# Card/Kanban View

*Note: macOS 13+ / iOS 16+ is required*

With **v3.16**, you can now view your notes not only as a table or list but also as **cards**. By grouping cards using a property from the [Frontmatter](237-frontmatter.md) of your notes, you can create a **Kanban View**, where the groups become columns.

Watch the video to learn how to get started:

## **Getting Started**

1. To create a Cards view of your notes, click on any folder in the left sidebar, which will open a table at first.
2. At the top you will see a toolbar with various options, change the view option from "List" to "Cards".
- macOS![](../../d33v4339jhl8k0.cloudfront.net/docs/assets/6081f7f4c9133261f23f4b41/images/678ee5c6d624882e7e78ecd1/file-Wo0cZ8TGyn.png)
- iOS
![](../../d33v4339jhl8k0.cloudfront.net/docs/assets/6081f7f4c9133261f23f4b41/images/678ee69bc474802bf4bca5b9/file-yhvR6vbF8d.png)
3. You will see all your notes as cards now.
4. Group your notes by some custom property, like "status" or "stage" that can serve as Kanban columns.
- If you haven't added any properties yet, it's time that you go into at least one note, add [Frontmatter](237-frontmatter.md) and a property such as "status: backlog".![](../../d33v4339jhl8k0.cloudfront.net/docs/assets/6081f7f4c9133261f23f4b41/images/677c59d42cb6c13419046143/file-mHTYChuIZZ.gif)
- Note: If you struggle typing three dashes, because it turns them into a long dash, you need to turn off "Smart Punctuation" (iOS) or "Smart Dashes" (macOS), see [details here](237-frontmatter.md).
- You will probably need to give more notes properties, so that you have more than just one column (+ one for "uncategorized").
- Now select the group in the "Group" dropdown.
- macOS
**![](../../d33v4339jhl8k0.cloudfront.net/docs/assets/6081f7f4c9133261f23f4b41/images/678ee61d8e63e96f9ad606b1/file-o3yGJW3QCZ.png)**
- iOS![](../../d33v4339jhl8k0.cloudfront.net/docs/assets/6081f7f4c9133261f23f4b41/images/678ee70c8e63e96f9ad606b3/file-fblbyfqHw9.png)

### Drag & Drop

You can **drag** cards between columns or rearrange them within a column. When you move a card between columns, NotePlan automatically updates the **group property** in the Frontmatter of the note.

**Custom Ordering with the “Order” Property
**A new "order" property is added to your notes to enable custom ordering of cards within a column. This property updates dynamically when you drag a card within a column or move it to another column. If any notes are missing the required frontmatter, NotePlan will prompt you to add it (you can choose to proceed or decline).

The Kanban view makes organizing and managing your tasks visually intuitive while keeping your notes synchronized with their metadata.

### Changing Properties

You an change properties directly from the cards by clicking on the property and selecting a different value. If you see just one option to select, you need to add more values to other notes first. This view picks up the existing values from all the notes it has access to.

![](../../d33v4339jhl8k0.cloudfront.net/docs/assets/6081f7f4c9133261f23f4b41/images/6793ac2bedd57570fcb17ab1/file-96cXRrfPKe.gif)

### Hiding Completed Cards

If you want to hide completed projects, you can add a property "completed" for example to your notes and use the filter feature to hide the card. Alternatively, you can also hide the complete "status: done" column, if you have one in your Kanban board.

Here's how the frontmatter looks like with a completed property for the note you want to hide:

```
---
status: Done
completed: yes
---
# Note Title
```

The notes you don't want to hide will need a "completed: no" property.

Now filter by "Completed", "Equals: no":

![](../../d33v4339jhl8k0.cloudfront.net/docs/assets/6081f7f4c9133261f23f4b41/images/67910b989298df7939597c9f/file-5r46xHx8RT.png)

Once every note has this property you can hide cards directly from the Cards View (to see the "no" drop down, enable the "Completed" property in the "Fields" dropdown):

![](../../d33v4339jhl8k0.cloudfront.net/docs/assets/6081f7f4c9133261f23f4b41/images/67910c7278756a282cee18f9/file-ZuOEWbQLOH.gif)

## Use Cases

What can you do with a Kanban board?

**Project Management
**Usually it's used to track tasks or projects through a process and visualize the current state, so it's clear where you are at the moment and what comes next. A typical Kanban process consists of three steps like 1. "ToDo", 2. "Doing" and 3. "Done".

[Learn more in this article](251-how-to-plan-with-the-kanban-board.md)

**Reading List![](../../d33v4339jhl8k0.cloudfront.net/docs/assets/6081f7f4c9133261f23f4b41/images/677c5d59cbe59814e288138c/file-54meTojJ03.png)**Not just formal projects, but other activities can be tracked like reading books or articles. This gives you a nice overview of what you have achieved already, what's next, what you have stopped (because you didn't like it), etc.

*Last updated: 2025-03-03*
