import type { Metadata } from "next";
import "./globals.css";

// No next/font/google on purpose: venue Wi-Fi is unreliable, so we use system stacks.
export const metadata: Metadata = {
  title: "Theseus",
  description: "C to Rust migration with behavioral evidence",
};

export default function RootLayout({ children }: LayoutProps<"/">) {
  return (
    <html lang="en" className="dark h-full antialiased">
      <body className="min-h-full bg-neutral-950 text-neutral-100">{children}</body>
    </html>
  );
}
