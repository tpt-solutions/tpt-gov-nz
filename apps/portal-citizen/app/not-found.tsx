import Link from "next/link";

export default function NotFound() {
  return (
    <section className="card">
      <h1 style={{ marginTop: 0 }}>Page not found</h1>
      <p>We couldn’t find the page you were looking for.</p>
      <Link href="/" className="btn">
        Back to home
      </Link>
    </section>
  );
}
