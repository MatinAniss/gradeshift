import Link from "next/link";
import NavbarInfo from "./navbarInfo";

export default function Navbar() {
  return (
    <nav className="sticky top-0 flex justify-between items-center bg-background h-14 w-full border-b z-50 border-border px-10">
      <Link href="/">
        <span>GradeShift</span>
      </Link>
      <NavbarInfo />
    </nav>
  );
}
