import * as React from "react";
import { createBrowserRouter, Link } from "react-router-dom";
import FileSystem from "../apps/Comments";
import KanbanBoard from "../apps/KanbanBoard";
import SearchBar from "../apps/SearchBar";
export const router = createBrowserRouter([
  {
    path: "/",
    element: (
      <div>
        <h1>Apps</h1>
        <ul>
          {["comments", "kanbanBoard", "searchBar"].map((e) => (
            <li key={e}>
              <Link to={e}>{e}</Link>
            </li>
          ))}
        </ul>
      </div>
    ),
  },
  {
    path: "/comments",
    element: <FileSystem />,
  },
  {
    path: "/kanbanBoard",
    element: <KanbanBoard />,
  },
  {
    path: "/searchBar",
    element: <SearchBar />,
  },
]);
