# grabber.zone Grabber

Automatically downloads comics from [grabber.zone](grabber.zone) and converts them into `.cbz` files.

## TODO

I think I can make this much easier.
Seemingly, all pages are like:

```html
<div class="page-break ">
    <img id="image-18" class="wp-manga-chapter-img img-responsive effect-fade lazyloaded" src=" https://grabber.zone/wp-content/uploads/WP-manga/data/manga_67dbf4d91a07e/37a3c8e0b803bc5b5515662bf919a30a/67269a1739590047b113727f1454781576.jpg">
</div>
```

So no difficult pattern matching is needed to figure out which resources to filter, and how to extract the page number.
We can just match on this specific structure.

Overall, the comic view looks as:

```html
<div class="entry-content">
    <div class="entry-content_wrap">
        <div class="read-container">
            <div class="reading-content">

				<input type="hidden" id="wp-manga-current-chap" data-id="4079" value="chapter-1">

                <div class="page-break ">
                    <img id="image-0" class="wp-manga-chapter-img img-responsive effect-fade lazyloaded" src=" https://grabber.zone/wp-content/uploads/WP-manga/data/manga_67dbf4d91a07e/37a3c8e0b803bc5b5515662bf919a30a/67269a1732155511b113727f793295508.png">
                </div>

                <div class="page-break ">
                    <img id="image-1" class="wp-manga-chapter-img img-responsive effect-fade lazyloaded" src=" https://grabber.zone/wp-content/uploads/WP-manga/data/manga_67dbf4d91a07e/37a3c8e0b803bc5b5515662bf919a30a/67269a1732155518b113727f1140490665.jpg">
                </div>

                ...

                <div class="page-break ">
                    <img id="image-41" class="wp-manga-chapter-img img-responsive effect-fade lazyloaded" src=" https://grabber.zone/wp-content/uploads/WP-manga/data/manga_67dbf4d91a07e/37a3c8e0b803bc5b5515662bf919a30a/67269a1747345465b113727f2002118714.jpg">
                </div>

            </div>
        </div>
    </div>
</div>
```

## TODO

Similarly for extracting issues from a collection.

```html
<div class="page-content-listing single-page">
    <div class="listing-chapters_wrap cols-4 show-more show">
        <ul class="main version-chap no-volumn active">

            <li class="wp-manga-chapter   has-thumb  ">
                <div class="chapter-thumbnail">
                    <a href="https://grabber.zone/comics/sonic-the-hedgehog-modern-era/sonic-the-hedgehog-247/">
                    <img class="thumb" src="https://grabber.zone/wp-content/uploads/thumb-1414-75x106.jpg">
                    </a>
                </div>
                <a href="https://grabber.zone/comics/sonic-the-hedgehog-modern-era/sonic-the-hedgehog-247/">
                </a><a href="https://grabber.zone/comics/sonic-the-hedgehog-modern-era/sonic-the-hedgehog-247/">
                Sonic the Hedgehog 247								</a>
                <span class="chapter-release-date">
                <i>November 24, 2021</i>									</span>
            </li>

            <li class="wp-manga-chapter   has-thumb  ">
                <div class="chapter-thumbnail">
                    <a href="https://grabber.zone/comics/sonic-the-hedgehog-modern-era/sonic-the-hedgehog-246/">
                    <img class="thumb" src="https://grabber.zone/wp-content/uploads/thumb-1413-75x106.jpg">
                    </a>
                </div>
                <a href="https://grabber.zone/comics/sonic-the-hedgehog-modern-era/sonic-the-hedgehog-246/">
                </a><a href="https://grabber.zone/comics/sonic-the-hedgehog-modern-era/sonic-the-hedgehog-246/">
                Sonic the Hedgehog 246								</a>
                <span class="chapter-release-date">
                <i>November 24, 2021</i>									</span>
            </li>

            <li class="wp-manga-chapter   has-thumb  ">
                <div class="chapter-thumbnail">
                    <a href="https://grabber.zone/comics/sonic-the-hedgehog-modern-era/sonic-the-hedgehog-160/">
                    <img class="thumb" src="https://grabber.zone/wp-content/uploads/thumb-1327-75x106.jpg">
                    </a>
                </div>
                <a href="https://grabber.zone/comics/sonic-the-hedgehog-modern-era/sonic-the-hedgehog-160/">
                </a><a href="https://grabber.zone/comics/sonic-the-hedgehog-modern-era/sonic-the-hedgehog-160/">
                Sonic the Hedgehog 160								</a>
                <span class="chapter-release-date">
                <i>November 24, 2021</i>									</span>
            </li>
        </ul>

        <div class="c-chapter-readmore">
            <span class="btn btn-link chapter-readmore less-chap" style="display: inline;">
            Show more 				</span>
        </div>
    </div>
</div>
```

It even has the pretty-printed title, and the release data.
We can use this in the metadata.

## TODO

This should then also change how we get a single issue, to simplify the approach and ensure we keep metadata.
Probably, still the collection url should be provided, but then there might be a flag `--latest` to get only the last issue, or the user might have to provide the index or the exact name match.
Not perfect, but makes my life a whole lot easier.
I expect `--latest` to be enough anyways. Typically, you will download an entire collection once, and then download new issues as they come out.
(Maybe it is possible to have `--latest <N>?` with an optional count, if clap supports that, to get the last N issues, instead of only the last one)
