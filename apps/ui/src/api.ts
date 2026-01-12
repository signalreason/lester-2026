export type Workspace = {
  id: string;
  name: string;
  created_at: number;
};

export type Bookmark = {
  id: string;
  workspace_id: string;
  url: string;
  title: string;
  notes?: string | null;
  created_at: number;
  updated_at: number;
};

export type Tag = {
  id: string;
  name: string;
  created_at: number;
};

export type TagJob = {
  id: string;
  bookmark_id: string;
  status: string;
  attempts: number;
  created_at: number;
  updated_at: number;
};

export type TagCloudEntry = {
  name: string;
  weight: number;
};

export type CreateBookmarkResponse = {
  bookmark: Bookmark;
  job: TagJob;
};

const API_URL = import.meta.env.VITE_API_URL ?? "http://127.0.0.1:7316";

async function request<T>(path: string, options?: RequestInit): Promise<T> {
  const response = await fetch(`${API_URL}${path}`, {
    headers: {
      "Content-Type": "application/json",
    },
    ...options,
  });

  if (!response.ok) {
    const payload = await response.json().catch(() => ({}));
    const message = payload?.error ?? `Request failed: ${response.status}`;
    throw new Error(message);
  }

  return (await response.json()) as T;
}

export function listWorkspaces(): Promise<Workspace[]> {
  return request("/workspaces");
}

export function createWorkspace(name: string): Promise<Workspace> {
  return request("/workspaces", {
    method: "POST",
    body: JSON.stringify({ name }),
  });
}

export function listBookmarks(params: {
  workspace_id?: string;
  tag?: string;
  q?: string;
} = {}): Promise<Bookmark[]> {
  const query = new URLSearchParams();
  if (params.workspace_id) query.set("workspace_id", params.workspace_id);
  if (params.tag) query.set("tag", params.tag);
  if (params.q) query.set("q", params.q);
  const suffix = query.toString() ? `?${query.toString()}` : "";
  return request(`/bookmarks${suffix}`);
}

export function createBookmark(input: {
  workspace_id: string;
  url: string;
  title: string;
  notes?: string;
}): Promise<CreateBookmarkResponse> {
  return request("/bookmarks", {
    method: "POST",
    body: JSON.stringify(input),
  });
}

export function listTags(): Promise<Tag[]> {
  return request("/tags");
}

export function listTagCloud(limit = 40): Promise<TagCloudEntry[]> {
  return request(`/tag-cloud?limit=${limit}`);
}
