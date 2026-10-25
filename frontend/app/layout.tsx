import type { Metadata } from "next";
import "./globals.css";

// Fonts are self-hosted in public/fonts (see globals.css): venue Wi-Fi is unreliable.
export const metadata: Metadata = {
  title: "Theseus",
  description: "C parsers to memory-safe Rust, with evidence a reviewer can rerun.",
};

export default function RootLayout({ children }: LayoutProps<"/">) {
  return (
    <html lang="en" className="dark h-full antialiased">
      <body className="min-h-full bg-ink text-paper">{children}</body>
    </html>
  );
}
