import { Outlet } from "react-router";
import SideBar from "./Sidebar";

export default function AppShell() {
  return (
    <div>
      <SideBar />

      <div>
        <Outlet />
      </div>
    </div>
  );
}
