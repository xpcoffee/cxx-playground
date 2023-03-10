#pragma once
#include <memory>

struct MultiBuf; // note we don't define internals; opaque to C++

class BlobstoreClient
{
public:
    BlobstoreClient();
    uint64_t put(MultiBuf &buf) const;
};

std::unique_ptr<BlobstoreClient> new_blobstore_client();
