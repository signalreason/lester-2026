import "./styles.css";
import {
  createBookmark,
  createWorkspace,
  listBookmarks,
  listTagCloud,
  listWorkspaces,
  type Bookmark,
  type TagCloudEntry,
  type Workspace,
} from "./api";

const state = {
  workspaces: [] as Workspace[],
  bookmarks: [] as Bookmark[],
  tagCloud: [] as TagCloudEntry[],
  selectedWorkspaceId: null as string | null,
  query: "",
  error: "",
};

const app = document.querySelector<HTMLDivElement>("#app");
if (!app) {
  throw new Error("Missing #app container");
}

async function loadData() {
  try {
    state.workspaces = await listWorkspaces();
    if (!state.selectedWorkspaceId && state.workspaces[0]) {
      state.selectedWorkspaceId = state.workspaces[0].id;
    }
    state.tagCloud = await listTagCloud();
    state.bookmarks = await listBookmarks({
      workspace_id: state.selectedWorkspaceId ?? undefined,
      q: state.query || undefined,
    });
    state.error = "";
  } catch (error) {
    state.error = error instanceof Error ? error.message : "Failed to load data";
  }
  render();
}

function render() {
  const workspaceButtons = state.workspaces
    .map((workspace) => {
      const isActive = workspace.id === state.selectedWorkspaceId;
      return `<button class="${isActive ? "active" : ""}" data-workspace="${workspace.id}">${escapeHtml(
        workspace.name,
      )}</button>`;
    })
    .join("");

  const tagCloudItems = state.tagCloud
    .map((entry) => {
      const size = Math.min(Math.max(entry.weight / 5, 0.6), 2.2);
      return `<span class="tag" style="--size: ${size.toFixed(2)}">${escapeHtml(
        entry.name,
      )}</span>`;
    })
    .join("");

  const bookmarkCards = state.bookmarks
    .map(
      (bookmark) => `
        <div class="bookmark">
          <strong>${escapeHtml(bookmark.title)}</strong>
          <small>${escapeHtml(bookmark.url)}</small>
          ${bookmark.notes ? `<div>${escapeHtml(bookmark.notes)}</div>` : ""}
        </div>
      `,
    )
    .join("");

  app.innerHTML = `
    <div class="app-shell">
      <aside class="rail">
        <section class="panel">
          <h2>Workspaces</h2>
          <div class="list">${workspaceButtons || "<p>No workspaces yet.</p>"}</div>
        </section>
        <section class="panel">
          <h2>Tag Cloud</h2>
          <div class="tag-cloud">${tagCloudItems || "<p>No tags yet.</p>"}</div>
        </section>
      </aside>
      <main class="main">
        <section class="panel">
          <h2>Research Intake</h2>
          <div class="actions">
            <form id="workspace-form">
              <label>
                New workspace
                <input name="workspace" placeholder="Quantum materials" />
              </label>
              <button class="primary" type="submit">Create</button>
            </form>
            <form id="bookmark-form">
              <label>
                Bookmark URL
                <input name="url" placeholder="https://example.com" />
              </label>
              <label>
                Title
                <input name="title" placeholder="Paper or article title" />
              </label>
              <label>
                Notes
                <textarea name="notes" rows="2" placeholder="Why it matters"></textarea>
              </label>
              <button class="primary" type="submit">Save Bookmark</button>
            </form>
          </div>
        </section>
        <section class="panel">
          <h2>Bookmarks</h2>
          <div class="search-row">
            <input id="search" placeholder="Search bookmarks" value="${escapeHtml(state.query)}" />
            <button class="primary" id="refresh">Refresh</button>
          </div>
          ${state.error ? `<p>${escapeHtml(state.error)}</p>` : ""}
          <div class="list">${bookmarkCards || "<p>No bookmarks yet.</p>"}</div>
        </section>
      </main>
    </div>
  `;

  wireEvents();
}

function wireEvents() {
  document.querySelectorAll<HTMLButtonElement>("[data-workspace]").forEach((button) => {
    button.addEventListener("click", async () => {
      state.selectedWorkspaceId = button.dataset.workspace ?? null;
      await loadData();
    });
  });

  const workspaceForm = document.querySelector<HTMLFormElement>("#workspace-form");
  workspaceForm?.addEventListener("submit", async (event) => {
    event.preventDefault();
    const form = event.currentTarget as HTMLFormElement;
    const value = (form.elements.namedItem("workspace") as HTMLInputElement).value.trim();
    if (!value) return;
    await createWorkspace(value);
    form.reset();
    await loadData();
  });

  const bookmarkForm = document.querySelector<HTMLFormElement>("#bookmark-form");
  bookmarkForm?.addEventListener("submit", async (event) => {
    event.preventDefault();
    if (!state.selectedWorkspaceId) {
      state.error = "Create a workspace before adding bookmarks.";
      render();
      return;
    }
    const form = event.currentTarget as HTMLFormElement;
    const url = (form.elements.namedItem("url") as HTMLInputElement).value.trim();
    const title = (form.elements.namedItem("title") as HTMLInputElement).value.trim();
    const notes = (form.elements.namedItem("notes") as HTMLTextAreaElement).value.trim();
    if (!url || !title) return;
    await createBookmark({
      workspace_id: state.selectedWorkspaceId,
      url,
      title,
      notes: notes || undefined,
    });
    form.reset();
    await loadData();
  });

  const searchInput = document.querySelector<HTMLInputElement>("#search");
  searchInput?.addEventListener("input", (event) => {
    state.query = (event.target as HTMLInputElement).value;
  });

  const refreshButton = document.querySelector<HTMLButtonElement>("#refresh");
  refreshButton?.addEventListener("click", async () => {
    await loadData();
  });
}

function escapeHtml(value: string): string {
  return value
    .replace(/&/g, "&amp;")
    .replace(/</g, "&lt;")
    .replace(/>/g, "&gt;")
    .replace(/\"/g, "&quot;")
    .replace(/'/g, "&#39;");
}

loadData();
