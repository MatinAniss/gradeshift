import NavbarInfo from "./navbarInfo";
import NavbarLogo from "./navbarLogo";

export default function Navbar() {
  return (
    <nav className="sticky top-0 flex justify-between items-center bg-background h-14 w-full border-b z-50 border-border px-10">
      <NavbarLogo />
      <NavbarInfo />
    </nav>
  );
}
