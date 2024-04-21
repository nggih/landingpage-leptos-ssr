use crate::error_template::{AppError, ErrorTemplate};
use leptos::*;
use leptos_meta::*;
use leptos_router::*;

#[component]
pub fn App() -> impl IntoView {
    // Provides context that manages stylesheets, titles, meta tags, etc.
    provide_meta_context();

    view! {
        <Stylesheet id="leptos" href="/pkg/landingpage.css"/>

        // sets the document title
        <Title text="Welcome to Leptos"/>

        // content for this welcome page
        <Router fallback=|| {
            let mut outside_errors = Errors::default();
            outside_errors.insert_with_default_key(AppError::NotFound);
            view! { <ErrorTemplate outside_errors/> }.into_view()
        }>
            <main>
                <Routes>
                    <Route path="" view=HomePage/>
                </Routes>
            </main>
        </Router>
    }
}

/// Renders the home page of your application.
#[component]
fn HomePage() -> impl IntoView {
    // Creates a reactive value to update the button
    // let (count, set_count) = create_signal(0);

    view! {
        <Title text="Leptos + Tailwindcss"/>
        <main>
            // <!-- Header Section -->
            <div class="bg-gray-900 text-white py-4">
                <div class="container mx-auto flex justify-between items-center">
                    <img src="company_logo.png" alt="Company Logo" class="h-8"/>
                    <nav>
                        <ul class="flex space-x-4">
                            <li>
                                <a href="#" class="hover:text-gray-300">
                                    Home
                                </a>
                            </li>
                            <li>
                                <a href="#about" class="hover:text-gray-300">
                                    About
                                </a>
                            </li>
                            <li>
                                <a href="#products" class="hover:text-gray-300">
                                    Products
                                </a>
                            </li>
                            <li>
                                <a href="#contact" class="hover:text-gray-300">
                                    Contact
                                </a>
                            </li>
                        </ul>
                    </nav>
                </div>
            </div>
            // <!-- Hero Section -->
            <section class="bg-gray-100 py-20 text-center">
                <div class="container mx-auto">
                    <h1 class="text-4xl font-bold mb-4">Quality XX Products</h1>
                    <p class="text-lg mb-8">
                        Explore our premium range of xx products sourced from the finest regions.
                    </p>
                    <a
                        href="#products"
                        class="bg-blue-500 hover:bg-blue-600 text-white font-bold py-2 px-4 rounded"
                    >
                        Explore Products
                    </a>
                </div>
            </section>

            // <!-- About Section -->
            <section id="about" class="py-16">
                <div class="container mx-auto">
                    <h2 class="text-2xl font-bold mb-8">About Us</h2>
                    <p class="mb-8">
                        Lorem ipsum dolor sit amet, consectetur adipiscing elit. Sed quis tempor nisi, ac tristique massa. Ut ultrices viverra diam eget viverra.
                    </p>
                    <p>
                        Lorem ipsum dolor sit amet, consectetur adipiscing elit. Sed quis tempor nisi, ac tristique massa.
                    </p>
                </div>
            </section>

            // <!-- Product Showcase -->
            <section id="products" class="bg-gray-100 py-16">
                <div class="container mx-auto">
                    <h2 class="text-2xl font-bold mb-8">Our Products</h2>
                    // <!-- Product Cards -->
                    <div class="grid grid-cols-1 md:grid-cols-3 gap-8">
                        // <!-- Product Card Example -->
                        <div class="bg-white shadow-md p-8 rounded-lg">
                            <img
                                src="product_image.jpg"
                                alt="Product"
                                class="h-32 w-auto mx-auto mb-4"
                            />
                            <h3 class="text-lg font-bold mb-2">Product Name</h3>
                            <p class="mb-4">Description of the product goes here.</p>
                            <p class="text-gray-600">Price: $XX.XX</p>
                        </div>
                    </div>
                </div>
            </section>

            // <!-- Contact Section -->
            <section id="contact" class="py-16">
                <div class="container mx-auto">
                    <h2 class="text-2xl font-bold mb-8">Contact Us</h2>
                    <p class="mb-4">
                        For inquiries or orders, please contact us through the following:
                    </p>
                    <ul class="list-disc pl-6">
                        <li>Email: info@example.com</li>
                        <li>Phone: +1 123-456-7890</li>
                    </ul>
                    // <!-- Business Inquiry Form -->
                    <form action="#" method="POST" class="mt-8">
                        <div class="grid grid-cols-1 md:grid-cols-2 gap-4">
                            <div>
                                <label for="name" class="block mb-2">
                                    Your Name
                                </label>
                                <input
                                    type="text"
                                    id="name"
                                    name="name"
                                    class="w-full px-4 py-2 border border-gray-300 rounded-md"
                                    required
                                />
                            </div>
                            <div>
                                <label for="email" class="block mb-2">
                                    Your Email
                                </label>
                                <input
                                    type="email"
                                    id="email"
                                    name="email"
                                    class="w-full px-4 py-2 border border-gray-300 rounded-md"
                                    required
                                />
                            </div>
                        </div>
                        <div class="mt-4">
                            <label for="message" class="block mb-2">
                                Your Message
                            </label>
                            <textarea
                                id="message"
                                name="message"
                                rows="4"
                                class="w-full px-4 py-2 border border-gray-300 rounded-md"
                                required
                            ></textarea>
                        </div>
                        <div class="mt-4">
                            <button
                                type="submit"
                                class="bg-blue-500 hover:bg-blue-600 text-white font-bold py-2 px-4 rounded"
                            >
                                Submit
                            </button>
                        </div>
                    </form>
                </div>
            </section>
            <a
                href="https://api.whatsapp.com/send?phone=1234567890"
                class="whatsapp-button fixed bottom-8 right-8 bg-green-500 hover:bg-green-600 text-white font-bold py-2 px-4 rounded-full"
            >
                <i class="fab fa-whatsapp">
                    // <!-- Whatsapp -->g
                    <span class="[&>svg]:h-7 [&>svg]:w-7 [&>svg]:fill-[#128c7e]">
                        <svg
                            xmlns="http://www.w3.org/2000/svg"
                            fill="currentColor"
                            viewBox="0 0 448 512"
                        >
                            <path d="M380.9 97.1C339 55.1 283.2 32 223.9 32c-122.4 0-222 99.6-222 222 0 39.1 10.2 77.3 29.6 111L0 480l117.7-30.9c32.4 17.7 68.9 27 106.1 27h.1c122.3 0 224.1-99.6 224.1-222 0-59.3-25.2-115-67.1-157zm-157 341.6c-33.2 0-65.7-8.9-94-25.7l-6.7-4-69.8 18.3L72 359.2l-4.4-7c-18.5-29.4-28.2-63.3-28.2-98.2 0-101.7 82.8-184.5 184.6-184.5 49.3 0 95.6 19.2 130.4 54.1 34.8 34.9 56.2 81.2 56.1 130.5 0 101.8-84.9 184.6-186.6 184.6zm101.2-138.2c-5.5-2.8-32.8-16.2-37.9-18-5.1-1.9-8.8-2.8-12.5 2.8-3.7 5.6-14.3 18-17.6 21.8-3.2 3.7-6.5 4.2-12 1.4-32.6-16.3-54-29.1-75.5-66-5.7-9.8 5.7-9.1 16.3-30.3 1.8-3.7 .9-6.9-.5-9.7-1.4-2.8-12.5-30.1-17.1-41.2-4.5-10.8-9.1-9.3-12.5-9.5-3.2-.2-6.9-.2-10.6-.2-3.7 0-9.7 1.4-14.8 6.9-5.1 5.6-19.4 19-19.4 46.3 0 27.3 19.9 53.7 22.6 57.4 2.8 3.7 39.1 59.7 94.8 83.8 35.2 15.2 49 16.5 66.6 13.9 10.7-1.6 32.8-13.4 37.4-26.4 4.6-13 4.6-24.1 3.2-26.4-1.3-2.5-5-3.9-10.5-6.6z"></path>
                        </svg>
                    </span>

                </i>
            </a>
        </main>
    }
}
