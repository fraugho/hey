import type { Metadata } from "next";
import "./globals.css";
import { Inter } from "next/font/google";
import { ThemeProvider } from "@/components/theme-provider"

export const metadata: Metadata = {
  title: "Hey",
  description: "Messaging Application",
};

const inter = Inter({ subsets: ["latin"] });
export default function RootLayout({
    children,
}: Readonly<{children: React.ReactNode;}>) {
    return (
        <html lang="en">
            <body className={inter.className}>
                <ThemeProvider
                    attribute="class"
                    defaultTheme="system"
                    enableSystem
                    disableTransitionOnChange
                >
                    {children}
                </ThemeProvider>
            </body>
        </html>
    );
}
