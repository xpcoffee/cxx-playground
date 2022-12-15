#include "cxx-demo/include/blobstore.h"
#include "cxx-demo/target/cxxbridge/cxx-demo/src/main.rs.h"
#include <functional>
#include <string>
#include <iostream>

BlobstoreClient::BlobstoreClient() {}

uint64_t BlobstoreClient::put(MultiBuf &buf) const
{
    std::string contents;
    while (true)
    {
        auto chunk = next_chunk(buf);
        if (chunk.size() == 0)
        {
            break;
        }
        contents.append(reinterpret_cast<const char *>(chunk.data()), chunk.size());
    }
    std::cout << std::string(buf.to_string());
    auto blobid = std::hash<std::string>{}(contents);
    return blobid;
}

std::unique_ptr<BlobstoreClient> new_blobstore_client()
{
    return std::unique_ptr<BlobstoreClient>(new BlobstoreClient());
}