glide.o.native_tabs = "hide";

type BookmarkFolder = {
  path: string;
  id: string;
};

const collectBookmarkFolders = (
  nodes: browser.bookmarks.BookmarkTreeNode[],
  prefix = "",
): BookmarkFolder[] => {
  const folders: BookmarkFolder[] = [];

  for (const node of nodes) {
    if (node.type !== "folder" && node.url) {
      continue;
    }

    const path = prefix ? `${prefix}/${node.title}` : node.title;
    folders.push({ path, id: node.id });

    if (node.children?.length) {
      folders.push(...collectBookmarkFolders(node.children, path));
    }
  }

  return folders;
};

const getBookmarkFolders = async (): Promise<BookmarkFolder[]> => {
  const [root] = await browser.bookmarks.getTree();
  return collectBookmarkFolders(root.children ?? []);
};

const resolveBookmarkFolder = (folders: BookmarkFolder[], path: string) => {
  const normalized = path.trim().toLowerCase();
  return folders.find((folder) => folder.path.toLowerCase() === normalized);
};

const saveBookmark = async (
  tab_id: number,
  parentId: string,
  title?: string,
) => {
  const tab = await browser.tabs.get(tab_id);
  const { url } = tab;
  if (!url?.startsWith("http")) {
    return;
  }

  const bookmarkTitle = title?.trim() || tab.title || url;
  const [existing] = await browser.bookmarks.search({ url });

  if (existing?.id) {
    await browser.bookmarks.update(existing.id, { title: bookmarkTitle });
    if (existing.parentId !== parentId) {
      await browser.bookmarks.move(existing.id, { parentId });
    }
    return;
  }

  await browser.bookmarks.create({ parentId, title: bookmarkTitle, url });
};

const showBookmarkFolderPicker = async (
  tab_id: number,
  title?: string,
) => {
  const folders = await getBookmarkFolders();

  glide.commandline.show({
    title: "bookmark folder",
    options: folders.map((folder) => ({
      label: folder.path,
      async execute() {
        await saveBookmark(tab_id, folder.id, title);
      },
    })),
  });
};

const bookmark_add = glide.excmds.create(
  {
    name: "bookmark_add",
    description: "Bookmark the current page",
    args_schema: {
      path: {
        type: "string",
        required: false,
        position: 0,
        description: "Bookmark folder path",
      },
    },
  },
  async ({ tab_id, args_arr }) => {
    const path = args_arr.join(" ").trim();

    if (!path) {
      await showBookmarkFolderPicker(tab_id);
      return;
    }

    const folders = await getBookmarkFolders();
    const folder = resolveBookmarkFolder(folders, path);

    if (!folder) {
      await showBookmarkFolderPicker(tab_id);
      return;
    }

    await saveBookmark(tab_id, folder.id);
  },
);

declare global {
  interface ExcmdRegistry {
    bookmark_add: typeof bookmark_add;
  }
}

glide.keymaps.set("visual", "e", async ({ tab_id }) => {
  const collapsed = await glide.content.execute(() => {
    const el = document.activeElement;
    if (el instanceof HTMLInputElement || el instanceof HTMLTextAreaElement) {
      return el.selectionStart === el.selectionEnd;
    }
    return window.getSelection()?.isCollapsed ?? true;
  }, { tab_id });

  if (collapsed) {
    // Match `motion vl`: include the anchor char before extending to word end.
    await glide.excmds.execute("motion vl");
  }
  await glide.excmds.execute("motion e");
}, { description: "Extend selection to end of word" });

glide.keymaps.set("normal", "gw", async () => {
  const tab = await glide.tabs.active();
  if (tab?.id !== undefined) {
    await browser.windows.create({ tabId: tab.id });
  }
}, { description: "Detach tab to new window" });

glide.keymaps.set("normal", "gb", async () => {
  const bookmarks = (await browser.bookmarks.search({}))
    .filter((bookmark) => bookmark.url)
    .map((bookmark) => ({
      title: bookmark.title || bookmark.url,
      url: bookmark.url,
    }));

  glide.commandline.show({
    title: "bookmarks",
    options: bookmarks.map((bookmark) => ({
      label: bookmark.title,
      description: bookmark.url,
      async execute() {
        const tab = await glide.tabs.get_first({ url: bookmark.url });
        if (tab?.id) {
          await browser.tabs.update(tab.id, { active: true });
        } else {
          await browser.tabs.create({ active: true, url: bookmark.url });
        }
      },
    })),
  });
}, { description: "Open bookmark picker" });
