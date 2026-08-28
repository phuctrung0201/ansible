glide.o.native_tabs = "hide";

glide.keymaps.set("visual", "e", "motion e");

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
}, { description: "[g]o to [b]ookmark" });
