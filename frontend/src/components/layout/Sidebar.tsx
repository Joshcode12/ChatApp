import { useState } from "react";
import UserBar from "./UserBar";

export default function Sidebar() {
  const [, setView] = useState<"dm" | "room">("dm");

  // fake data for now
  const fakeItems: string[] = ["general", "random", "off-topic"];

  return (
    <aside>
      <div>
        <button onClick={() => setView("dm")}>Direct Messages</button>
        <button onClick={() => setView("room")}>Rooms</button>
      </div>

      <ul>
        {fakeItems.map((name) => (
          <li key={name}>
            <button>{name}</button>
          </li>
        ))}
      </ul>

      <UserBar />
    </aside>
  );
}
